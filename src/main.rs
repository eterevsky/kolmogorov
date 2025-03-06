mod bf_count;
mod bf_num0;
mod brainfuck;
mod def;
mod stat;
mod turing_count;

use crate::def::{System, Generator, ProgResult};

fn run<CS: System>(comp: &CS, max_size: usize) {
    let mut gen = comp.generate(max_size);
    let mut stat: stat::Stat<CS> = stat::Stat::new();
    let mut max_steps = 0;

    while let Some((program, weight)) = gen.next() {
        let result = comp.execute(&program, std::cmp::max(1000, 4 * max_steps));
        let new = stat.register(&program, &result, weight);
        if let ProgResult::Out { output, steps } = result {
            if steps > max_steps {
                max_steps = steps;
                println!("Max steps: {}", max_steps);
            }

            if new {
                println!("{}  {}", output, program);
            }
        }
    }

    stat.print();
}

fn main() {
    // run(&bf_count::BfCount::new(), 11);
    // run(&bf_num0::BfNum0::new(), 11);
    run(&turing_count::TuringCount::new(), 4);
}
