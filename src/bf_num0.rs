use crate::brainfuck::{BfGenerator, BfProgram, BfInstruction};
use crate::def::{CompSystem, ProgResult};

#[derive(Debug)]
pub struct BfNum0 {
}

impl BfNum0 {
    pub fn new() -> Self {
        BfNum0 {}
    }

    fn maybe_extend_tape(tape: &mut Vec<i32>, pos: usize) {
        while pos >= tape.len() {
            tape.push(0);
        }
    }
}

impl CompSystem for BfNum0 {
    type Output = u64;
    type Program = BfProgram;

    fn generate(&self) -> BfGenerator {
        BfGenerator::new(true, false)
    }

    fn execute(&self, program: &BfProgram, max_steps: usize) -> ProgResult<u64> {
        let mut step = 0;
        let mut tape = vec![0];
        let mut pos = 0;
        let mut ip = 0;

        while step < max_steps && ip < program.0.len() {
            let inst = program.0[ip];
            match inst {
                BfInstruction::Plus => {
                    Self::maybe_extend_tape(&mut tape, pos);
                    tape[pos] += 1;
                    ip += 1;
                }
                BfInstruction::Minus => {
                    Self::maybe_extend_tape(&mut tape, pos);
                    tape[pos] -= 1;
                    ip += 1;
                }
                BfInstruction::Left => {
                    if pos == 0 {
                        return ProgResult::Error;
                    }
                    pos -= 1;
                    ip += 1;
                }
                BfInstruction::Right => {
                    pos += 1;
                    ip += 1;
                }
                BfInstruction::StartLoop(offset) => {
                    Self::maybe_extend_tape(&mut tape, pos);
                    ip += if tape[pos] != 0 { 1 } else { offset }
                }
                BfInstruction::EndLoop(offset) => {
                    ip -= offset;
                }
                _ => ()
            }
            step += 1;
        }

        if step >= max_steps {
            ProgResult::Timeout
        } else if tape[0] > 0 {
            ProgResult::Out { output: tape[0] as u64, steps: step }
        } else {
            ProgResult::Error
        }
    }
}
