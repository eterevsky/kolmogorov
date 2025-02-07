const BFBIN_LENGTH_LIMIT: usize = 12;


// Count the number of valid binary BF programs without `[]` and `++`
const fn bfbin_counts() -> [u64; BFBIN_LENGTH_LIMIT] {
    let mut counts = [0; BFBIN_LENGTH_LIMIT];
    let mut counts_not_from_plus = [0; BFBIN_LENGTH_LIMIT];

    counts[0] = 1;
    counts_not_from_plus[0] = 1;

    let mut len = 1;
    while len < BFBIN_LENGTH_LIMIT {
        // . < >
        let mut count = 3 * counts[len - 1];

        // [ ... ] ...
        let mut inner_len = 1;
        while inner_len + 2 <= len {
            count += counts[inner_len] * counts[len - inner_len - 2];
            inner_len += 1;
        }

        counts_not_from_plus[len] = count;

        // + ...
        count += counts_not_from_plus[len - 1];

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

fn main() {
    println!("{:?}", BFBIN_COUNTS);
    println!("{:?}", BFBIN_CUMULATIVE);
}

