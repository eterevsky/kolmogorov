use crate::def::{Generator, ProgResult, Sized, System};

#[derive(Clone, Copy)]
struct TuringCountRule {
    new_state: usize,
    tape_value: bool,
    move_right: bool,
}

impl std::fmt::Display for TuringCountRule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.new_state, self.tape_value as usize)?;
        if self.move_right {
            write!(f, ">")
        } else {
            write!(f, "<")
        }
    }
}

const STATE_NAMES: [char; 10] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
const DIRECTIONS: [&'static str; 2] = ["<", ">"];

fn get_state_name(nstates: usize, state: usize) -> char {
    if state == nstates {
        'Z'
    } else {
        STATE_NAMES[state]
    }
}

#[derive(Clone)]
pub struct TuringCountProgram {
    // Number of non-terminal states. Terminal state is #nstates
    nstates: usize,
    rules: Vec<[TuringCountRule; 2]>,
}

impl Sized for TuringCountProgram {
    fn size(&self) -> usize {
        self.nstates
    }
}

impl std::fmt::Display for TuringCountProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.nstates {
            for v in 0..2 {
                let rule = self.rules[i][v];
                write!(f, " {}{}:", get_state_name(self.nstates, i), v)?;
                if rule.new_state == self.nstates {
                    write!(f, "HALT")?;
                } else {
                    write!(
                        f,
                        "{}{}{}",
                        rule.tape_value as usize,
                        DIRECTIONS[rule.move_right as usize],
                        get_state_name(self.nstates, rule.new_state)
                    )?;
                }
            }
        }
        std::fmt::Result::Ok(())
    }
}

pub struct TuringCountGenerator {
    index_within_nstates: usize,
    total_for_nstates: usize,
    max_states: usize,
    nstates: usize,
}

impl TuringCountGenerator {
    fn new(max_states: usize) -> Self {
        TuringCountGenerator {
            max_states,
            index_within_nstates: 0,
            total_for_nstates: 0,
            nstates: 0,
        }
    }

    fn increment_nstates(&mut self) {
        self.nstates += 1;
        self.index_within_nstates = 0;
        self.total_for_nstates = 1;

        for _ in 0..self.nstates {
            self.total_for_nstates *= (4 * self.nstates + 1) * (4 * self.nstates + 1);
        }
        println!(
            "nstates: {}, total machines: {}",
            self.nstates, self.total_for_nstates
        );
    }

    fn rule_from_idx(&self, idx: &mut usize) -> TuringCountRule {
        let rule_idx = *idx % (4 * self.nstates + 1);
        *idx /= 4 * self.nstates + 1;

        let new_state = rule_idx / 4;
        let tape_value = (rule_idx & 2) != 0;
        let move_right = (rule_idx & 1) != 0;

        TuringCountRule {
            new_state,
            tape_value,
            move_right,
        }
    }
}

impl Generator<TuringCountProgram> for TuringCountGenerator {
    fn next(&mut self) -> Option<(TuringCountProgram, usize)> {
        if self.index_within_nstates >= self.total_for_nstates {
            self.increment_nstates();
        }
        if self.nstates > self.max_states {
            return None;
        }
        let mut idx = self.index_within_nstates;
        let mut program = TuringCountProgram {
            nstates: self.nstates,
            rules: Vec::new(),
        };
        for _istate in 0..self.nstates {
            let rule_for_0 = self.rule_from_idx(&mut idx);
            let rule_for_1 = self.rule_from_idx(&mut idx);

            program.rules.push([rule_for_0, rule_for_1]);
        }
        self.index_within_nstates += 1;

        Some((program, 1))
    }
}

pub struct TuringCount {}

impl TuringCount {
    pub fn new() -> Self {
        TuringCount {}
    }
}

impl System for TuringCount {
    type Output = u64;
    type Program = TuringCountProgram;

    fn execute(&self, program: &Self::Program, max_steps: usize) -> ProgResult<u64> {
        let mut tape_positive = vec![false];
        let mut tape_negative = vec![false];
        let mut position: i32 = 0;
        let mut output = 0;
        let mut state = 0;
        let mut step = 0;

        while step < max_steps && state < program.nstates {
            let tape_value = if position >= 0 {
                while tape_positive.len() <= position as usize {
                    tape_positive.push(false);
                }
                tape_positive[position as usize] as usize
            } else {
                let pos = (-position - 1) as usize;
                while tape_negative.len() <= pos {
                    tape_negative.push(false);
                }
                tape_negative[pos] as usize
            };

            let rule = &program.rules[state][tape_value];

            state = rule.new_state;
            if position >= 0 {
                tape_positive[position as usize] = rule.tape_value;
            } else {
                tape_negative[(-position - 1) as usize] = rule.tape_value;
            };

            if rule.move_right {
                position += 1;
            } else {
                position -= 1;
            }

            if state == 0 {
                output += 1;
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

    fn generate(&self, limit: usize) -> TuringCountGenerator {
        TuringCountGenerator::new(limit)
    }

    fn valid_output(o: &u64) -> bool {
        *o > 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::def::Generator;
    use crate::stat::Stat;

    struct TuringCountNaiveGenerator {
        index_within_nstates: usize,
        total_for_nstates: usize,
        max_states: usize,
        nstates: usize,
    }

    impl TuringCountNaiveGenerator {
        fn new(max_states: usize) -> Self {
            TuringCountNaiveGenerator {
                max_states,
                index_within_nstates: 0,
                total_for_nstates: 0,
                nstates: 0,
            }
        }

        fn increment_nstates(&mut self) {
            self.nstates += 1;
            self.index_within_nstates = 0;
            self.total_for_nstates = 1;

            for _ in 0..self.nstates {
                self.total_for_nstates *= 16 * (self.nstates + 1) * (self.nstates + 1);
            }
            println!(
                "nstates: {}, total machines: {}",
                self.nstates, self.total_for_nstates
            );
        }

        fn rule_from_idx(&self, idx: &mut usize) -> TuringCountRule {
            let new_state = *idx % (self.nstates + 1);
            *idx /= self.nstates + 1;
            let tape_value = (*idx % 2) != 0;
            *idx /= 2;
            let move_right = (*idx % 2) != 0;
            *idx /= 2;

            TuringCountRule {
                new_state,
                tape_value,
                move_right,
            }
        }

        fn rule_is_valid(&self, rule: &TuringCountRule) -> bool {
            rule.new_state < self.nstates || (!rule.tape_value && !rule.move_right)
        }
    }

    impl Generator<TuringCountProgram> for TuringCountNaiveGenerator {
        fn next(&mut self) -> Option<(TuringCountProgram, usize)> {
            'idx: loop {
                if self.index_within_nstates >= self.total_for_nstates {
                    self.increment_nstates();
                }
                if self.nstates > self.max_states {
                    return None;
                }
                let mut idx = self.index_within_nstates;
                self.index_within_nstates += 1;
                let mut program = TuringCountProgram {
                    nstates: self.nstates,
                    rules: Vec::new(),
                };
                for _istate in 0..self.nstates {
                    let rule_for_0 = self.rule_from_idx(&mut idx);
                    if !self.rule_is_valid(&rule_for_0) {
                        // Don't distinguish between different rules that lead to HALT state
                        continue 'idx;
                    }
                    let rule_for_1 = self.rule_from_idx(&mut idx);
                    if !self.rule_is_valid(&rule_for_1) {
                        continue 'idx;
                    }

                    program.rules.push([rule_for_0, rule_for_1]);
                }

                return Some((program, 1));
            }
        }
    }

    #[test]
    fn results_match() {
        let comp = TuringCount::new();

        for max_size in 1..=3 {
            let mut gen1 = comp.generate(max_size);
            let mut stat1: Stat<TuringCount> = Stat::new();

            while let Some((program, weight)) = gen1.next() {
                let result = comp.execute(&program, 100);
                stat1.register(&program, &result, weight);
            }

            let mut gen2 = TuringCountNaiveGenerator::new(max_size);
            let mut stat2: Stat<TuringCount> = Stat::new();

            while let Some((program, weight)) = gen2.next() {
                let result = comp.execute(&program, 100);
                stat2.register(&program, &result, weight);
            }

            println!("stat1:");
            stat1.print();
            println!("\nstat2:");
            stat2.print();

            assert!(stat1.matches_outputs(&stat2));
        }
    }
}
