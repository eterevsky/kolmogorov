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

fn main() {
    println!("{:?}", BFBIN_COUNTS);
    println!("{:?}", BFBIN_CUMULATIVE);

    println!("{} {}", 123457, bfbin_from_idx(1234567));
}
