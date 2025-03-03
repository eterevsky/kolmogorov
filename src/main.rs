mod bf_count;
mod bf_num0;
mod brainfuck;
mod def;
mod stat;
mod turing_count;

use bf_count::BfCount;

use crate::def::{CompSystem2, Generator};

fn main() {
    let comp = BfCount::new();
    let mut gen = comp.generate(9);
    let mut stat: stat::Stat<BfCount> = stat::Stat::new();

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

    return;
}
