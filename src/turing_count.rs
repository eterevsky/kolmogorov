use crate::def::{CompSystem, ProgGenerator, ProgResult};

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
            write!(f, "->")
        } else {
            write!(f, "<-")
        }
    }
}

const STATE_NAMES: [char; 10] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
const DIRECTIONS: [&'static str; 2] = ["<-", "->"];

fn get_state_name(nstates: usize, state: usize) -> char {
    if state == nstates {
        'Z'
    } else {
        STATE_NAMES[state]
    }
}

pub struct TuringCountProgram {
    // Number of non-terminal states. Terminal state is #nstates
    nstates: usize,
    rules: Vec<[TuringCountRule; 2]>,
}

impl std::fmt::Display for TuringCountProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.nstates {
            for v in 0..2 {
                let rule = self.rules[i][v];
                write!(f, " ({}{}  {}", get_state_name(self.nstates, i), v, get_state_name(self.nstates, rule.new_state))?; 
                if rule.new_state != self.nstates {
                    write!(f, "{}{}", rule.tape_value as usize, DIRECTIONS[rule.move_right as usize])?; 
                }
                write!(f, ")")?;
            }
        }
        std::fmt::Result::Ok(())
    }
}

pub struct TuringCountGenerator {
    index_within_nstates: usize,
    total_for_nstates: usize,
    current: TuringCountProgram,
}

impl TuringCountGenerator {
    fn new() -> Self {
        TuringCountGenerator {
            index_within_nstates: 0,
            total_for_nstates: 0,
            current: TuringCountProgram {
                nstates: 0,
                rules: Vec::new(),
            }
        }
    }

    fn increment_nstates(&mut self) {
        self.current.nstates += 1;
        self.index_within_nstates = 0;
        self.total_for_nstates = 1;

        for _ in 0..self.current.nstates {
            self.total_for_nstates *= 16 * (self.current.nstates + 1) * (self.current.nstates + 1);
        }
        println!("nstates: {}, total machines: {}", self.current.nstates, self.total_for_nstates);
    }

    fn rule_from_idx(&self, idx: &mut usize) -> TuringCountRule {
        let new_state = *idx % (self.current.nstates + 1);
        *idx /= self.current.nstates + 1;
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
}

impl ProgGenerator<TuringCountProgram> for TuringCountGenerator {
    fn next<'a>(&'a mut self) -> &'a TuringCountProgram {
        if self.index_within_nstates >= self.total_for_nstates {
            self.increment_nstates();
        }
        let mut idx = self.index_within_nstates;
        self.current.rules.clear();
        for _istate in 0..self.current.nstates {
            let rule_for_0 = self.rule_from_idx(&mut idx);
            let rule_for_1 = self.rule_from_idx(&mut idx);

            self.current.rules.push([rule_for_0, rule_for_1]);
        }
        self.index_within_nstates += 1;

        &self.current
    }
}

pub struct TuringCount {}

impl TuringCount {
    pub fn new() -> Self {
        TuringCount {}
    }
}

impl CompSystem for TuringCount {
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
            ProgResult::Out { output, steps: step }
        }
    }

    fn generate(&self) -> TuringCountGenerator {
        TuringCountGenerator::new()
    }
}
