mod bf_count;
mod brainfuck;
mod def;

use std::collections::HashSet;

use crate::bf_count::BfCount;
use crate::def::{CompSystem, ProgGenerator, ProgResult};

fn main() {
    let comp = BfCount::new();
    let mut generator = comp.generate();
    let mut generated = HashSet::new();
    let mut steps_limit= 100;
    let mut max_steps = 0;

    for i in 0..1_000_000_000_000 as usize {
    // for i in 0..100 {
        let program = generator.next();
        let result = comp.execute(program, steps_limit);
        // println!("{} {} {:?}", i, program, result);

        if let ProgResult::Out { output, steps } = result {
            if steps > max_steps {
                max_steps = steps;
                println!("Max steps: {}", max_steps);
                steps_limit = max_steps * 4;
            }
            if !generated.contains(&output) {
                println!("{} {} {}", i, program, output);
                generated.insert(output.clone());
            }
        }
    }
}
