use std::{fmt::Display, hash::Hash};

#[derive(Debug)]
pub enum ProgResult<Output> {
    Out { output: Output, steps: usize },
    Error,
    Timeout,
}

pub trait Generator<Program> {
    fn next(&mut self) -> Option<(Program, usize)>;
}

pub trait Sized {
    fn size(&self) -> usize;
}

pub trait System {
    type Output: Display + PartialEq + Eq + Hash + PartialOrd + Ord;
    type Program: Clone + Display + Sized;

    // Generate the valid programs.
    fn generate(&self, limit: usize) -> impl Generator<Self::Program>;

    fn execute(&self, program: &Self::Program, max_steps: usize) -> ProgResult<Self::Output>;

    fn valid_output(o: &Self::Output) -> bool;
}
