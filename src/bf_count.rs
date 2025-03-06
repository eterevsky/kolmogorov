use crate::brainfuck::{
    BfNaiveGenerator, BfRawInstruction, BfSource,
};
use crate::def::{System, ProgResult};
use arrayvec::ArrayVec;

#[derive(Clone, Copy)]
enum Instruction {
    Flip,
    Inc,
    Left,
    Right,
    StartLoop(usize),
    EndLoop(usize),
}

#[derive(Debug)]
pub struct BfCount {}

impl BfCount {
    pub fn new() -> Self {
        BfCount {}
    }

    fn compile(source: &BfSource) -> ArrayVec<Instruction, 28> {
        let mut open_loops = ArrayVec::<usize, 28>::new();
        let mut program = ArrayVec::new();

        for &instruction in source.0.iter() {
            let inst = match instruction {
                BfRawInstruction::Print => Instruction::Inc,
                BfRawInstruction::Plus => Instruction::Flip,
                BfRawInstruction::Minus => unreachable!(),
                BfRawInstruction::Left => Instruction::Left,
                BfRawInstruction::Right => Instruction::Right,
                BfRawInstruction::StartLoop => {
                    open_loops.push(program.len());
                    Instruction::StartLoop(0)
                }
                BfRawInstruction::EndLoop => {
                    let open = open_loops.pop().unwrap();
                    program[open] = Instruction::StartLoop(program.len() + 1);
                    Instruction::EndLoop(open)
                }
            };
            program.push(inst);
        }

        program
    }

    fn maybe_extend_tape(tape: &mut Vec<bool>, pos: usize) {
        while pos >= tape.len() {
            tape.push(false);
        }
    }
}

impl System for BfCount {
    type Output = u64;
    type Program = BfSource;

    fn generate(&self, limit: usize) -> BfNaiveGenerator {
        BfNaiveGenerator::new(limit, false, true)
    }

    fn execute(&self, source: &BfSource, max_steps: usize) -> ProgResult<u64> {
        let program = Self::compile(source);

        let mut step = 0;
        let mut tape = Vec::new();
        let mut pos = 0;
        let mut output = 0;
        let mut ip = 0;

        while step < max_steps && ip < program.len() {
            let inst = program[ip];
            match inst {
                Instruction::Flip => {
                    Self::maybe_extend_tape(&mut tape, pos);
                    tape[pos] = !tape[pos];
                    ip += 1;
                }
                Instruction::Inc => {
                    output += 1;
                    ip += 1;
                }
                Instruction::Left => {
                    if pos == 0 {
                        return ProgResult::Error;
                    }
                    pos -= 1;
                    ip += 1;
                }
                Instruction::Right => {
                    pos += 1;
                    ip += 1;
                }
                Instruction::StartLoop(target) => {
                    Self::maybe_extend_tape(&mut tape, pos);
                    if tape[pos] {
                        ip += 1
                    } else {
                        ip = target
                    }
                }
                Instruction::EndLoop(target) => {
                    ip = target;
                }
            }
            step += 1;
        }

        if step >= max_steps {
            ProgResult::Timeout
        } else {
            ProgResult::Out {
                output,
                steps: step,
            }
        }
    }

    fn valid_output(output: &u64) -> bool {
        *output > 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::stat::Stat;
    use crate::def::Generator;

#[test]
fn results_match() {
    let comp = BfCount::new();

    for max_size in 1..9 {
        let mut gen1 = comp.generate(max_size);
        let mut stat1: Stat<BfCount> = Stat::new();

        while let Some((program, weight)) = gen1.next() {
            let result = comp.execute(&program, 1000);
            stat1.register(&program, &result, weight);
        }

        let mut gen2 = BfNaiveGenerator::new(max_size, false, true);
        let mut stat2: Stat<BfCount> = Stat::new();

        while let Some((program, weight)) = gen2.next() {
            let result = comp.execute(&program, 1000);
            stat2.register(&program, &result, weight);
        }

        assert!(stat1.matches_outputs(&stat2));
    }
}

}