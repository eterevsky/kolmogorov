use crate::def::{CompSystem2, ProgResult};
use std::collections::HashMap;

struct OutputStat<C: CompSystem2> {
    min_program: C::Program,
    count: usize,
}

pub struct Stat<C: CompSystem2> {
    error: usize,
    timeout: usize,
    outputs: HashMap<C::Output, OutputStat<C>>,
}

impl<C: CompSystem2> Stat<C> {
    pub fn new() -> Self {
        Stat {
            error: 0,
            timeout: 0,
            outputs: HashMap::new(),
        }
    }

    pub fn register(&mut self, program: &C::Program, result: ProgResult<C::Output>, weight: usize) {
        match result {
            ProgResult::Error => {
                self.error += 1;
            }
            ProgResult::Timeout => {
                self.timeout += 1;
            }
            ProgResult::Out { output, steps: _ } => {
                let mut entry = self.outputs.entry(output).or_insert(OutputStat {
                    min_program: program.clone(),
                    count: 0,
                });
                entry.count += weight;
            }
        }
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
    }
}
