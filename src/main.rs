mod bf_count;
mod bf_num0;
mod brainfuck;
mod def;
mod stat;
mod turing_count;

use bf_count::BfCount;

use crate::def::{CompSystem2, Generator};

fn run<CS: CompSystem2>(comp: &CS, max_size: usize) {
    let mut gen = comp.generate(max_size);
    let mut stat: stat::Stat<CS> = stat::Stat::new();

    while let Some((program, weight)) = gen.next() {
        let result = comp.execute(&program, 1000);
        // match result {
        //     ProgResult::Error => println!("{}  Error", program),
        //     ProgResult::Timeout => println!("{}  Timeout", program),
        //     ProgResult::Out { output, steps } => {
        //         println!("{}  {} in {} steps", program, output, steps)
        //     }
        // };
        stat.register(&program, result, weight);
    }

    stat.print();
}

fn main() {
    // run(&bf_count::BfCount::new(), 11);
    run(&bf_num0::BfNum0::new(), 11);
}
