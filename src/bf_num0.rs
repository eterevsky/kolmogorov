use crate::brainfuck::{
    BfNaiveGenerator, BfRawInstruction, BfSource,
};
use crate::def::{CompSystem2, ProgResult};
use arrayvec::ArrayVec;

#[derive(Clone, Copy)]
enum Instruction {
    Plus,
    Minus,
    Left,
    Right,
    StartLoop(usize),
    EndLoop(usize),
}

#[derive(Debug)]
pub struct BfNum0 {
}

impl BfNum0 {
    pub fn new() -> Self {
        BfNum0 {}
    }

    fn compile(source: &BfSource) -> ArrayVec<Instruction, 28> {
        let mut open_loops = ArrayVec::<usize, 28>::new();
        let mut program = ArrayVec::new();

        for &instruction in source.0.iter() {
            let inst = match instruction {
                BfRawInstruction::Plus => Instruction::Plus,
                BfRawInstruction::Minus => Instruction::Minus,
                BfRawInstruction::Print => unreachable!(),
                BfRawInstruction::Left => Instruction::Left,
                BfRawInstruction::Right => Instruction::Right,
                BfRawInstruction::StartLoop => {
                    open_loops.push(program.len());
                    Instruction::StartLoop(0)
                }
                BfRawInstruction::EndLoop => {
                    let open = open_loops.pop().unwrap();
                    program[open] = Instruction::StartLoop(program.len() + 1);
                    Instruction::EndLoop(open)
                }
            };
            program.push(inst);
        }

        program
    }

    fn maybe_extend_tape(tape: &mut Vec<i32>, pos: usize) {
        while pos >= tape.len() {
            tape.push(0);
        }
    }
}

impl CompSystem2 for BfNum0 {
    type Output = i64;
    type Program = BfSource;

    fn valid_output(o: &i64) -> bool {
        *o > 0
    }

    fn generate(&self, limit: usize) -> BfNaiveGenerator {
        BfNaiveGenerator::new(limit, true, false)
    }

    fn execute(&self, source: &BfSource, max_steps: usize) -> ProgResult<i64> {
        let program = Self::compile(source);

        let mut step = 0;
        let mut tape = vec![0];
        let mut pos = 0;
        let mut ip = 0;

        while step < max_steps && ip < program.len() {
            let inst = program[ip];
            match inst {
                Instruction::Plus => {
                    Self::maybe_extend_tape(&mut tape, pos);
                    tape[pos] += 1;
                    ip += 1;
                }
                Instruction::Minus => {
                    Self::maybe_extend_tape(&mut tape, pos);
                    tape[pos] -= 1;
                    ip += 1;
                }
                Instruction::Left => {
                    if pos == 0 {
                        return ProgResult::Error;
                    }
                    pos -= 1;
                    ip += 1;
                }
                Instruction::Right => {
                    pos += 1;
                    ip += 1;
                }
                Instruction::StartLoop(target) => {
                    Self::maybe_extend_tape(&mut tape, pos);
                    if tape[pos] != 0 {
                        ip += 1
                    } else {
                        ip = target
                    }
                }
                Instruction::EndLoop(target) => {
                    ip = target;
                }
            }
            step += 1;
        }

        if step >= max_steps {
            ProgResult::Timeout
        } else if tape[0] > 0 {
            ProgResult::Out { output: tape[0] as i64, steps: step }
        } else {
            ProgResult::Error
        }
    }
}

mod test {
    use super::*;
    use crate::stat::Stat;
    use crate::def::Generator;

#[test]
fn results_match() {
    let comp = BfNum0::new();

    for max_size in 1..9 {
        let mut gen1 = comp.generate(max_size);
        let mut stat1: Stat<BfNum0> = Stat::new();

        while let Some((program, weight)) = gen1.next() {
            let result = comp.execute(&program, 1000);
            stat1.register(&program, result, weight);
        }

        let mut gen2 = BfNaiveGenerator::new(max_size, true, false);
        let mut stat2: Stat<BfNum0> = Stat::new();

        while let Some((program, weight)) = gen2.next() {
            let result = comp.execute(&program, 1000);
            stat2.register(&program, result, weight);
        }

        assert!(stat1.matches_outputs(&stat2));
    }
}

}