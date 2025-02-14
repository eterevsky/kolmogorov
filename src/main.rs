use arrayvec::ArrayString;

const BFBIN_LENGTH_LIMIT: usize = 12;


// Count the number of valid binary BF programs without `[]`
const fn bfbin_counts() -> [u64; BFBIN_LENGTH_LIMIT] {
    let mut counts = [0; BFBIN_LENGTH_LIMIT];

    counts[0] = 1;

    let mut len = 1;
    while len < BFBIN_LENGTH_LIMIT {
        // . + < >
        let mut count = 4 * counts[len - 1];

        // [ ... ] ...
        let mut inner_len = 1;
        while inner_len + 2 <= len {
            count += counts[inner_len] * counts[len - inner_len - 2];
            inner_len += 1;
        }

        counts[len] = count;
        len += 1;
    }

    counts
}

const BFBIN_COUNTS: [u64; BFBIN_LENGTH_LIMIT] = bfbin_counts();

const fn bfbin_cumulative() -> [u64; BFBIN_LENGTH_LIMIT + 1] {
    let mut cumulative = [0; BFBIN_LENGTH_LIMIT + 1];
    let mut l = 0;
    while l < BFBIN_LENGTH_LIMIT {
        cumulative[l + 1] = cumulative[l] + BFBIN_COUNTS[l];
        l += 1;
    }

    cumulative
}

const BFBIN_CUMULATIVE: [u64; BFBIN_LENGTH_LIMIT + 1] = bfbin_cumulative();

fn bfbin_from_len_idx(len: usize, idx: u64) -> ArrayString<BFBIN_LENGTH_LIMIT> {


    if len == 0 {
        return ArrayString::new()
    }

    let prev_count = BFBIN_COUNTS[len - 1];

    const SINGLE_FIRST: [char; 4] = ['.', '+', '<', '>'];

    for i in 0..4 {
        if idx < prev_count * (i + 1) {
            let mut program = ArrayString::new();
            program.push(SINGLE_FIRST[i as usize]);
            program.push_str(&bfbin_from_len_idx(len - 1, idx - i * prev_count));
            return program;
        }
    }


    let loop_idx = idx - 4 * prev_count;
    let mut cumulative = 0;

    let mut inner_len = 1;
    loop {
        let count = BFBIN_COUNTS[inner_len] * BFBIN_COUNTS[len - inner_len - 2];
        let next_cumulative = cumulative + count;
        if loop_idx < next_cumulative {
            break;
        }

        cumulative = next_cumulative;
        inner_len += 1;
    }

    let inner_outer_idx = loop_idx - cumulative;
    let outer_idx = inner_outer_idx / BFBIN_COUNTS[inner_len];
    let inner_idx = inner_outer_idx % BFBIN_COUNTS[inner_len];

    let mut program = ArrayString::new();

    program.push('[');
    program.push_str(&bfbin_from_len_idx(inner_len, inner_idx));
    program.push(']');
    program.push_str(&bfbin_from_len_idx(len - inner_len - 2, outer_idx));

    program
}

fn bfbin_from_idx(idx: u64) -> ArrayString<BFBIN_LENGTH_LIMIT> {
    let mut len = 0;
    while idx >= BFBIN_CUMULATIVE[len + 1] {
        len += 1;
    }

    let idx_in_len = idx - BFBIN_CUMULATIVE[len];
    // eprintln!("idx = {}, len = {}, idx_in_len = {}", idx, len, idx_in_len);

    bfbin_from_len_idx(len, idx_in_len)
}

#[derive(Debug)]
enum Outcome {
    Error,
    Timeout,
    Out(Vec<bool>),
}

#[derive(Debug)]
struct Result {
    steps: usize,
    outcome: Outcome,
}

// . + < >
#[derive(Clone, Copy, Debug)]
enum BfBinInstruction {
  Print,
  Plus,
  Left,
  Right,
  StartLoop(usize),
  EndLoop(usize),
}

#[derive(Debug)]
struct BfBinProgram {
    instructions: Vec<BfBinInstruction>
}

fn bfbin_compile(source: &str) -> BfBinProgram {
    let mut open_loops = Vec::new();
    let mut program = BfBinProgram { instructions: Vec::new() };
    for c in source.chars() {
        let inst = match c {
            '.' => BfBinInstruction::Print,
            '+' => BfBinInstruction::Plus,
            '<' => BfBinInstruction::Left,
            '>' => BfBinInstruction::Right,
            '[' => {
                open_loops.push(program.instructions.len());
                BfBinInstruction::StartLoop(0)
            },
            ']' => {
                let start = open_loops.pop().unwrap();
                program.instructions[start] = BfBinInstruction::StartLoop(program.instructions.len());
                BfBinInstruction::EndLoop(start)
            },
            _ => continue,
        };
        program.instructions.push(inst);
    }

    program
}

fn bfbin_maybe_extend_tape(tape: &mut Vec<bool>, pos: usize) {
    while pos >= tape.len() {
        tape.push(false);
    }
}

fn bfbin_run(source: &str, max_steps: usize) -> Result {
    let program = bfbin_compile(source);
    let mut step = 0;
    let mut tape = Vec::new();
    let mut output = Vec::new();
    let mut pos = 0;
    let mut ip = 0;

    while step < max_steps && ip < program.instructions.len() {
        let inst = program.instructions[ip];
        match inst {
            BfBinInstruction::Print => { 
                bfbin_maybe_extend_tape(&mut tape, pos);
                output.push(tape[pos]);
                ip += 1;
            },
            BfBinInstruction::Plus => {
                bfbin_maybe_extend_tape(&mut tape, pos);
                tape[pos] = !tape[pos];
                ip += 1;
            },
            BfBinInstruction::Left => {
                if pos == 0 {
                    return Result {
                        steps: step,
                        outcome: Outcome::Error,
                    }
                }
                pos -= 1;
                ip += 1;
            },
            BfBinInstruction::Right => {
                pos += 1;
                ip += 1;
            },
            BfBinInstruction::StartLoop(end) => {
                bfbin_maybe_extend_tape(&mut tape, pos);
                ip = if tape[pos] {
                    ip + 1
                } else {
                    end + 1
                }
            },
            BfBinInstruction::EndLoop(start) => {
                ip = start;
            }
        }
    }

    if step >= max_steps {
        Result {
            steps: step,
            outcome: Outcome::Timeout
        }
    } else {
        Result {
            steps: step,
            outcome: Outcome::Out(output)
        }
    }
}

fn main() {
    println!("{:?}", BFBIN_COUNTS);
    println!("{:?}", BFBIN_CUMULATIVE);

    let prog_source = bfbin_from_idx(1234567);
    println!("{} {}", 123457, prog_source);

    let prog = bfbin_compile(&prog_source);
    println!("{:?}", &prog);

    let out = bfbin_run(".+.>[.]+", 1000);
    println!("{:?}", &out);
}
