use crate::def::{System, ProgResult, Sized};
use std::collections::HashMap;

pub struct OutputStat<C: System> {
    pub min_program: C::Program,
    pub count: usize,
}

pub struct Stat<C: System> {
    error: usize,
    timeout: usize,
    invalid_output: usize,
    pub outputs: HashMap<C::Output, OutputStat<C>>,
    runs: Vec<usize>,
}

impl<C: System> Stat<C> {
    pub fn new() -> Self {
        Stat {
            error: 0,
            timeout: 0,
            invalid_output: 0,
            outputs: HashMap::new(),
            runs: Vec::new(),
        }
    }

    pub fn register(&mut self, program: &C::Program, result: &ProgResult<C::Output>, weight: usize) -> bool {
        let size = program.size();
        while size >= self.runs.len() {
            self.runs.push(0);
        }
        self.runs[size] += 1;
        let mut new = false;
        match result {
            ProgResult::Error => {
                self.error += weight;
            }
            ProgResult::Timeout => {
                self.timeout += weight;
            }
            ProgResult::Out { ref output, steps: _ } => {
                if C::valid_output(output) {
                    let output: C::Output = (*output).clone();
                    let entry = self.outputs.entry(output).or_insert_with(|| {
                        new = true;
                        OutputStat {
                            min_program: program.clone(),
                            count: 0,
                        }
                    }
                    );
                    entry.count += weight;
                } else {
                    self.invalid_output += weight;
                }
            }
        }

        new
    }

    #[cfg(test)]
    pub fn matches_outputs(&self, other: &Self) -> bool {
        for (out, count1) in self.outputs.iter() {
            let count2 = other.outputs.get(out).map(|os| os.count);
            if count2 != Some(count1.count) {
                return false;
            }
        }

        for (out, count2) in other.outputs.iter() {
            let count1 = self.outputs.get(out).map(|os| os.count);
            if count1 != Some(count2.count) {
                return false;
            }
        }

        true
    }

    pub fn print(&self) {
        println!("Errors: {}", self.error);
        println!("Timeout: {}", self.timeout);
        let mut entries: Vec<_> = self.outputs.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        let total = entries.iter().map(|(_, v)| v.count).sum::<usize>() as f64;
        for (o, s) in entries {
            println!(
                "{}  {:.2}  {}  {}",
                o,
                -(s.count as f64 / total).log2(),
                &s.min_program,
                s.count,
            );
        }
        println!("Runs:");
        for i in 0..self.runs.len() {
            if self.runs[i] > 0 {
                println!("{}  {}", i, self.runs[i]);
            }
        }
    }
}
