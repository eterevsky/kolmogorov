use crate::def::{ProgGenerator, ProgResult};

#[derive(Clone, Copy, Debug)]
pub enum BfInstruction {
    Print,
    Plus,
    Minus,
    Left,
    Right,
    // The parameter is the offset from the current instruction to the first
    // instruction after the loop.
    StartLoop(usize),
    // The parameter is the absolute value of the offset from the current
    // instruction to [.
    EndLoop(usize),
}

#[derive(Debug)]
pub struct BfProgram(pub Vec<BfInstruction>);

impl std::fmt::Display for BfProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for &inst in self.0.iter() {
            match inst {
                BfInstruction::Print => write!(f, ".")?,
                BfInstruction::Plus => write!(f, "+")?,
                BfInstruction::Minus => write!(f, "-")?,
                BfInstruction::Left => write!(f, "<")?,
                BfInstruction::Right => write!(f, ">")?,
                BfInstruction::StartLoop(_) => write!(f, "[")?,
                BfInstruction::EndLoop(_) => write!(f, "]")?,
            }
        }
        std::fmt::Result::Ok(())
    }
}

pub struct BfGenerator {
    max_idx: usize,
    cur_idx: usize,
    cur_len: usize,
    non_loop_instructions: Vec<BfInstruction>,
    // Count of all Brainfuck programs of a given length.
    len_counts: Vec<usize>,
    cumulative_counts: Vec<usize>,
}

impl BfGenerator {
    pub fn new( max_idx: usize, has_minus: bool, has_print: bool) -> Self {
        let mut len_counts = vec![1];
        let mut cumulative_counts = vec![0, 1];
        let mut non_loop_instructions = vec![
            BfInstruction::Left,
            BfInstruction::Right,
            BfInstruction::Plus,
        ];

        if has_minus {
            non_loop_instructions.push(BfInstruction::Minus);
        }

        if has_print {
            non_loop_instructions.push(BfInstruction::Print);
        }

        let mut len = 1;
        while *len_counts.last().unwrap() <= max_idx {
            let mut count = non_loop_instructions.len() * len_counts[len - 1];

            // [ ... ] ...
            let mut inner_len = 0;
            while inner_len + 2 <= len {
                count += len_counts[inner_len] * len_counts[len - inner_len - 2];
                inner_len += 1;
            }

            len_counts.push(count);
            cumulative_counts.push(cumulative_counts[len] + count);
            len += 1;
        }

        BfGenerator {
            max_idx,
            cur_idx: 0,
            cur_len: 0,
            len_counts,
            cumulative_counts,
            non_loop_instructions,
        }
    }

    fn from_len_idx(&self, len: usize, idx: usize) -> BfProgram {
        if len == 0 {
            return BfProgram(Vec::new());
        }

        let prev_count = self.len_counts[len - 1];

        for i in 0..self.non_loop_instructions.len() {
            if idx < prev_count * (i + 1) {
                let mut program = self.from_len_idx(len - 1, idx - i * prev_count);
                program.0.insert(0, self.non_loop_instructions[i]);
                return program;
            }
        }

        let loop_idx = idx - self.non_loop_instructions.len() * prev_count;
        let mut cumulative = 0;

        let mut inner_len = 0;
        loop {
            let count = self.len_counts[inner_len] * self.len_counts[len - inner_len - 2];
            let next_cumulative = cumulative + count;
            if loop_idx < next_cumulative {
                break;
            }

            cumulative = next_cumulative;
            inner_len += 1;
        }

        let inner_outer_idx = loop_idx - cumulative;
        let outer_idx = inner_outer_idx / self.len_counts[inner_len];
        let inner_idx = inner_outer_idx % self.len_counts[inner_len];

        let mut program = self.from_len_idx(inner_len, inner_idx);
        for inst in program.0.iter_mut() {
            match *inst {
                BfInstruction::StartLoop(n) => {
                    *inst = BfInstruction::StartLoop(n + 1);
                }
                BfInstruction::EndLoop(n) => {
                    *inst = BfInstruction::EndLoop(n + 1);
                }
                _ => (),
            }
        }
        program
            .0
            .insert(0, BfInstruction::StartLoop(inner_len + 2));
        program.0.push(BfInstruction::EndLoop(inner_len + 1));

        let head_len = program.0.len();

        let tail = self.from_len_idx(len - inner_len - 2, outer_idx);
        program.0.extend_from_slice(&tail.0);

        program
    }
}

impl ProgGenerator<BfProgram> for BfGenerator {
    fn next(&mut self) -> Option<(usize, BfProgram)> {
        let cur_idx = self.cur_idx;
        self.cur_idx += 1;

        if cur_idx >= self.max_idx {
            return None;
        }

        if cur_idx >= self.cumulative_counts[self.cur_len + 1] {
            self.cur_len += 1;
        }

        Some((
            cur_idx,
            self.from_len_idx(
                self.cur_len,
                cur_idx - self.cumulative_counts[self.cur_len],
            ),
        ))
    }

    fn register_result<O>(&mut self, _program: &BfProgram, _result: &ProgResult<O>) {}
}
