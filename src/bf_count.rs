use crate::brainfuck::{BfGenerator, BfProgram, BfInstruction};
use crate::def::{CompSystem, ProgResult};

#[derive(Debug)]
pub struct BfCount {
}

impl BfCount {
    pub fn new() -> Self {
        BfCount {}
    }

    fn maybe_extend_tape(tape: &mut Vec<bool>, pos: usize) {
        while pos >= tape.len() {
            tape.push(false);
        }
    }
}

impl CompSystem for BfCount {
    type Output = u64;
    type Program = BfProgram;

    fn generate(&self) -> BfGenerator {
        BfGenerator::new(false, true)
    }

    fn execute(&self, program: &BfProgram, max_steps: usize) -> ProgResult<u64> {
        let mut step = 0;
        let mut tape = Vec::new();
        let mut pos = 0;
        let mut output = 0;
        let mut ip = 0;

        while step < max_steps && ip < program.0.len() {
            let inst = program.0[ip];
            match inst {
                BfInstruction::Print => {
                    output += 1;
                    ip += 1;
                }
                BfInstruction::Plus => {
                    Self::maybe_extend_tape(&mut tape, pos);
                    tape[pos] = !tape[pos];
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
                    ip += if tape[pos] { 1 } else { offset }
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
        } else {
            ProgResult::Out { output, steps: step }
        }
    }
}
