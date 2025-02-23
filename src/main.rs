mod bf_count;
mod brainfuck;
mod def;

use std::collections::HashSet;

use crate::bf_count::BfCount;
use crate::def::{CompSystem, ProgGenerator, ProgResult};

// fn bfbin_maybe_extend_tape(tape: &mut Vec<bool>, pos: usize) {
// }

// fn bfbin_run(source: &str, max_steps: usize) -> SeqResult {
//     let program = bfbin_compile(source);
//     let mut step = 0;
//     let mut tape = Vec::new();
//     let mut output = Vec::new();
//     let mut pos = 0;
//     let mut ip = 0;

//     while step < max_steps && ip < program.instructions.len() {
//         let inst = program.instructions[ip];
//         match inst {
//             BfBinInstruction::Print => {
//                 bfbin_maybe_extend_tape(&mut tape, pos);
//                 output.push(tape[pos]);
//                 ip += 1;
//             }
//             BfBinInstruction::Plus => {
//                 bfbin_maybe_extend_tape(&mut tape, pos);
//                 tape[pos] = !tape[pos];
//                 ip += 1;
//             }
//             BfBinInstruction::Left => {
//                 if pos == 0 {
//                     return SeqResult {
//                         steps: step,
//                         outcome: SeqOutcome::Error,
//                     };
//                 }
//                 pos -= 1;
//                 ip += 1;
//             }
//             BfBinInstruction::Right => {
//                 pos += 1;
//                 ip += 1;
//             }
//             BfBinInstruction::StartLoop(end) => {
//                 bfbin_maybe_extend_tape(&mut tape, pos);
//                 ip = if tape[pos] { ip + 1 } else { end + 1 }
//             }
//             BfBinInstruction::EndLoop(start) => {
//                 ip = start;
//             }
//         }
//         step += 1;
//     }

//     if step >= max_steps {
//         SeqResult {
//             steps: step,
//             outcome: SeqOutcome::Timeout,
//         }
//     } else {
//         SeqResult {
//             steps: step,
//             outcome: SeqOutcome::Out(output),
//         }
//     }
// }

// fn bfcount_run(source: &str, max_steps: usize) -> NumResult {
//     let program = bfbin_compile(source);
//     let mut step = 0;
//     let mut tape = Vec::new();
//     let mut output = 0;
//     let mut pos = 0;
//     let mut ip = 0;

// }

fn main() {
    let comp = BfCount::new(1000);
    let mut generator = comp.generate(1_000_000_000);
    let mut generated = HashSet::new();

    while let Some((idx, program)) = generator.next() {
        let result = comp.execute(&program);
        // println!("{} {} {:?}", idx, &program, result);

        if let ProgResult::Out{output, steps: _} = result {
            if !generated.contains(&output) {
                println!("{} {} {}", idx, program, output);
                generated.insert(output.clone());
            }
        }
    }
}
