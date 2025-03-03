mod bf_count;
mod bf_num0;
mod brainfuck;
mod def;
mod turing_count;

use std::{collections::HashMap};

use crate::def::{CompSystem, ProgGenerator, ProgResult};

fn main() {
    // let comp = bf_count::BfCount::new();
    let comp = bf_num0::BfNum0::new();
    // let comp = turing_count::TuringCount::new();
    let mut generator = comp.generate();
    let mut generated = HashMap::new();
    let mut steps_limit= 200;
    let mut max_steps = 0;

    for i in 0..50_000_000_000 as usize {
    // for i in 0..1_000_000_000 {
        let program = generator.next();
        let result = comp.execute(program, steps_limit);
        // println!("{} {} {:?}", i, program, result);

        if let ProgResult::Out { output, steps } = result {
            if steps > max_steps {
                max_steps = steps;
                println!("Max steps: {}", max_steps);
                steps_limit = max_steps * 4;
            }

            *generated.entry(output).or_insert_with(|| {
                println!("{} {} {}", i, program, output);
                0
            }) += 1;
        }
    }

    let mut entries: Vec<_> = generated.iter().collect();
    entries.sort();
    let total = entries.iter().map(|(_, &v)| v).sum::<usize>() as f64;

    for (&n, &c) in entries {
        let l = if n > 0 { (n as f64).log2() } else {0.0};
        println!("{},{:.1}", n, -(c as f64 / total).log2());
    }
}
