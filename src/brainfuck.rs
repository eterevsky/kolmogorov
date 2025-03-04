use crate::def::{Generator, ProgGenerator, Sized};
use arrayvec::ArrayVec;

// So that ArrayString of this size fit in 32 bytes.
const BF_MAX_LEN: usize = 28;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub
enum BfRawInstruction {
    Plus,
    Minus,
    Print,
    Left,
    Right,
    StartLoop,
    EndLoop,
}

#[derive(Clone, Debug)]
pub struct BfSource(pub ArrayVec<BfRawInstruction, BF_MAX_LEN>);

impl std::fmt::Display for BfSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for &inst in self.0.iter() {
            match inst {
                BfRawInstruction::Print => write!(f, ".")?,
                BfRawInstruction::Plus => write!(f, "+")?,
                BfRawInstruction::Minus => write!(f, "-")?,
                BfRawInstruction::Left => write!(f, "<")?,
                BfRawInstruction::Right => write!(f, ">")?,
                BfRawInstruction::StartLoop => write!(f, "[")?,
                BfRawInstruction::EndLoop => write!(f, "]")?,
            }
        }
        std::fmt::Result::Ok(())
    }
}

impl Sized for BfSource {
    fn size(&self) -> usize {
        self.0.len()
    }
}

pub struct BfNaiveGenerator {
    instructions: ArrayVec<BfRawInstruction, 7>,
    max_len: usize,
    len: usize,
    idx: usize,
    total_for_len: usize,
}

impl BfNaiveGenerator {
    pub fn new(max_len: usize, has_minus: bool, has_print: bool) -> Self {
        let mut instructions = ArrayVec::new();
        instructions.push(BfRawInstruction::Plus);
        if has_minus {
            instructions.push(BfRawInstruction::Minus);
        }
        if has_print {
            instructions.push(BfRawInstruction::Print);
        }
        instructions.push(BfRawInstruction::Left);
        instructions.push(BfRawInstruction::Right);
        instructions.push(BfRawInstruction::StartLoop);
        instructions.push(BfRawInstruction::EndLoop);
        BfNaiveGenerator {
            instructions,
            max_len,
            len: 0,
            idx: 0,
            total_for_len: 0,
        }
    }

    fn inc_len(&mut self) {
        self.len += 1;
        self.total_for_len = 1;
        for _ in 0..self.len {
            self.total_for_len *= self.instructions.len();
        }
        self.idx = 0;
    }
}

impl Generator<BfSource> for BfNaiveGenerator {
    fn next(&mut self) -> Option<(BfSource, usize)> {
        let mut program = BfSource(ArrayVec::new());
        'idx: loop {
            program.0.clear();

            if self.idx >= self.total_for_len {
                self.inc_len();
            }

            if self.len > self.max_len {
                return None;
            }

            let mut idx = self.idx;
            self.idx += 1;

            let mut open_loops = 0;
            for _ in 0..self.len {
                let instruction = self.instructions[idx % self.instructions.len()];
                idx /= self.instructions.len();
                match instruction {
                    BfRawInstruction::EndLoop => open_loops += 1,
                    BfRawInstruction::StartLoop => {
                        if open_loops == 0 {
                            continue 'idx;
                        };
                        open_loops -= 1;
                    }
                    _ => (),
                }
                program.0.push(instruction);
            }

            if open_loops == 0 {
                program.0.reverse();
                return Some((program, 1));
            }
        }
    }
}
