use crate::brainfuck::{BfGenerator, BfProgram, BfInstruction};
use crate::def::{CompSystem, ProgResult};

#[derive(Debug)]
pub struct BfCount {
    max_steps: usize
}

impl BfCount {
    pub fn new(max_steps: usize) -> Self {
        BfCount { max_steps }
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

    fn generate(&self, max_idx: usize) -> BfGenerator {
        BfGenerator::new(max_idx, false, true)
    }

    fn execute(&self, program: &BfProgram) -> ProgResult<u64> {
        let mut step = 0;
        let mut tape = Vec::new();
        let mut pos = 0;
        let mut output = 0;
        let mut ip = 0;

        while step < self.max_steps && ip < program.0.len() {
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

        if step >= self.max_steps {
            ProgResult::Timeout
        } else {
            ProgResult::Out { output, steps: step }
        }
    }
}
