use crate::def::{CompSystem, ProgGenerator, ProgResult};

struct TuringCountRule {
    new_state: usize,
    move_right: bool,
    tape_value: bool,
    increment_output: bool,
}

struct TuringCountProgram {
    // Number of non-terminal states. Terminal state is #nstates
    nstates: usize,
    rules: Vec<[TuringCountRule; 2]>,
}

struct TuringCountGenerator {
    nstates: usize,
    index_within_nstates: usize,
    current: TuringCountProgram,
}

impl ProgGenerator<TuringCountProgram> for TuringCountGenerator {
    fn next
}

struct TuringCount {}

impl CompSystem for TuringCount {
    type Output = u64;
    type Program = TuringCountProgram;

    fn execute(&self, program: &Self::Program, max_steps: usize) -> ProgResult<u64> {
        let mut tape_positive = vec![false];
        let mut tape_negative = vec![false];
        let mut position: i32 = 0;
        let mut output = 0;
        let mut state = 0;
        let mut ip = 0;
        let mut step = 0;

        while step < max_steps && state < program.nstates {
            let tape_value = if position >= 0 {
                while tape_positive.len() <= position {
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

            let rule = program.rules[state][tape_value];

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

            if rule.increment_output {
                output += 1;
            }

            step += 1;
        }

        if step >= max_steps {
            ProgResult::Timeout
        } else {
            ProgResult::Out { output, steps: step }
        }
    }

    fn generate(&self) -> TuringCountGenerator {
        TuringCountGenerator::new()
    }
}