use crate::def::{Generator, ProgGenerator};
use arrayvec::ArrayVec;

// So that ArrayString of this size fit in 32 bytes.
const BF_MAX_LEN: usize = 28;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BfRawInstruction {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BfInstruction {
    Right,
    Left,
    Plus,
    Minus,
    Print,
    // The parameter is the offset from the current instruction to the first
    // instruction after the loop.
    StartLoop(usize),
    // The parameter is the absolute value of the offset from the current
    // instruction to [.
    EndLoop(usize),
}

// +-.<>[]

impl BfInstruction {
    fn idx(self) -> usize {
        match self {
            BfInstruction::Plus => 0,
            BfInstruction::Minus => 1,
            BfInstruction::Print => 2,
            BfInstruction::Left => 3,
            BfInstruction::Right => 4,
            BfInstruction::StartLoop(_) => 5,
            BfInstruction::EndLoop(_) => 6,
        }
    }
}

#[derive(Debug)]
pub struct BfProgram(pub Vec<BfInstruction>);

impl BfProgram {
    fn fix_loops(&mut self) {
        let mut starts = Vec::new();
        let mut pos = 0;

        while pos < self.0.len() {
            let inst = self.0[pos];
            match inst {
                BfInstruction::StartLoop(_) => starts.push(pos),
                BfInstruction::EndLoop(_) => {
                    let start = starts.pop().unwrap();
                    self.0[start] = BfInstruction::StartLoop(pos + 1 - start);
                    self.0[pos] = BfInstruction::EndLoop(pos - start);
                }
                _ => (),
            }
            pos += 1;
        }
    }
}

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

struct BfSettings {
    has_minus: bool,
    next_inst: [Option<BfInstruction>; 8],
}

impl BfSettings {
    fn new(has_minus: bool, has_print: bool) -> Self {
        assert!(BfInstruction::Plus.idx() == 0);
        let mut instructions = vec![BfInstruction::Plus];

        if has_minus {
            instructions.push(BfInstruction::Minus);
        }

        if has_print {
            instructions.push(BfInstruction::Print);
        }
        instructions.push(BfInstruction::Left);
        instructions.push(BfInstruction::Right);
        instructions.push(BfInstruction::StartLoop(0));
        instructions.push(BfInstruction::EndLoop(0));

        let mut next_inst = [None; 8];

        for i in 0..instructions.len() - 1 {
            let inst = instructions[i];
            next_inst[inst.idx()] = Some(instructions[i + 1]);
        }

        BfSettings {
            has_minus,
            next_inst,
        }
    }

    fn first(&self) -> BfInstruction {
        BfInstruction::Plus
    }

    fn next(&self, inst: BfInstruction) -> Option<BfInstruction> {
        self.next_inst[inst.idx()]
    }
}

fn next_slice(
    settings: &BfSettings,
    slice: &mut [BfInstruction],
    start_pos: usize,
) -> Option<usize> {
    let mut pos = slice.len();
    let mut open_loops = 0;

    while pos > 0 {
        pos -= 1;
        let inst = slice[pos];
        match inst {
            BfInstruction::EndLoop(_) => {
                open_loops += 1;
            }
            BfInstruction::StartLoop(_) => {
                open_loops -= 1;
            }
            _ => (),
        }

        if pos >= start_pos {
            continue;
        }

        let mut maybe_next_inst = settings.next(inst);
        while maybe_next_inst.is_some() {
            let next_inst = maybe_next_inst.unwrap();

            if open_loops == 0 {
                if let BfInstruction::EndLoop(_) = next_inst {
                    maybe_next_inst = settings.next(next_inst);
                    continue;
                }
            }

            let open_loops_after = match next_inst {
                BfInstruction::StartLoop(_) => open_loops + 1,
                BfInstruction::EndLoop(_) => open_loops - 1,
                _ => open_loops,
            };
            let remaining_positions = slice.len() - pos - 1;
            if remaining_positions < open_loops_after {
                maybe_next_inst = settings.next(next_inst);
                continue;
            }

            slice[pos] = next_inst;

            for i in (pos + 1)..(slice.len() - open_loops_after) {
                slice[i] = settings.first();
            }

            for i in (slice.len() - open_loops_after)..slice.len() {
                slice[i] = BfInstruction::EndLoop(0);
            }
            return Some(pos);
        }
    }

    None
}

fn next_program(
    settings: &BfSettings,
    program: &mut Vec<BfInstruction>,
    start_pos: usize,
) -> usize {
    let updated = next_slice(settings, &mut program[..], start_pos);

    if let Some(pos) = updated {
        pos
    } else {
        for i in 0..program.len() {
            program[i] = settings.first();
        }
        program.push(settings.first());
        0
    }
}

pub struct BfGenerator {
    settings: BfSettings,
    current: BfProgram,
    started: bool,
}

impl BfGenerator {
    pub fn new(has_minus: bool, has_print: bool) -> Self {
        BfGenerator {
            settings: BfSettings::new(has_minus, has_print),
            current: BfProgram(Vec::new()),
            started: false,
        }
    }
}

impl ProgGenerator<BfProgram> for BfGenerator {
    fn next<'a>(&'a mut self) -> &'a BfProgram {
        if !self.started {
            self.started = true;
            return &self.current;
        }

        let len = self.current.0.len();
        let mut min_modified = next_program(&self.settings, &mut self.current.0, len);

        let mut unverified = true;

        while unverified {
            let min_affected = if min_modified == 0 {
                0
            } else {
                min_modified - 1
            };

            unverified = false;

            let len = self.current.0.len();
            // for i in min_affected..(len - 1) {
            //     if self.current.0[i] == BfInstruction::Left
            //         && self.current.0[i + 1] == BfInstruction::Right
            //         || self.current.0[i] == BfInstruction::Right
            //             && self.current.0[i + 1] == BfInstruction::Left
            //         || (!self.settings.has_minus
            //             && self.current.0[i] == BfInstruction::Plus
            //             && self.current.0[i + 1] == BfInstruction::Plus)
            //         || (self.current.0[i] == BfInstruction::Minus
            //             && self.current.0[i + 1] == BfInstruction::Plus)
            //         || (self.current.0[i] == BfInstruction::Plus
            //             && self.current.0[i + 1] == BfInstruction::Minus)
            //     {
            //         // println!("next1 {} {}", &self.current, i + 2);
            //         let modified = next_program(&self.settings, &mut self.current.0, i + 2);
            //         if modified < min_modified {
            //             min_modified = modified;
            //         }
            //         // println!("next2 {} {}", &self.current, i + 2);
            //         unverified = true;
            //         break;
            //     }
            // }

            if unverified {
                continue;
            }

            if min_modified == 0 {
                let first_inst = *self.current.0.first().unwrap();
                match first_inst {
                    BfInstruction::Left | BfInstruction::StartLoop(_) => {
                        // println!("next1 {} {}", &self.current, 1);
                        next_program(&self.settings, &mut self.current.0, 1);
                        // println!("next2 {} {}", &self.current, 1);
                        unverified = true;
                    }
                    _ => (),
                }
            }
        }

        self.current.fix_loops();

        &self.current
    }

    // fn register_result<O>(&mut self, _program: &BfProgram, _result: &ProgResult<O>) {}
}
