use std::{fmt::Display, hash::Hash};

#[derive(Debug)]
pub enum ProgResult<Output> {
    Out { output: Output, steps: usize },
    Error,
    Timeout,
}

pub trait ProgGenerator<Program> {
    fn next<'a>(&'a mut self) -> &'a Program;
}

// New generator of programs
pub trait Generator<Program> {
    fn next(&mut self) -> Option<(Program, usize)>;
}

pub trait Sized {
    fn size(&self) -> usize;
}

pub trait CompSystem {
    type Output;
    type Program;

    // Generate the valid programs with their indices. The programs that result
    // in an error, or are generating the same output as the shorter programs,
    // can be skipped.
    fn generate(&self) -> impl ProgGenerator<Self::Program>;

    fn execute(&self, program: &Self::Program, max_steps: usize) -> ProgResult<Self::Output>;
}

pub trait CompSystem2 {
    type Output: Display + PartialEq + Eq + Hash + PartialOrd + Ord;
    type Program: Clone + Display + Sized;

    // Generate the valid programs.
    fn generate(&self, limit: usize) -> impl Generator<Self::Program>;

    fn execute(&self, program: &Self::Program, max_steps: usize) -> ProgResult<Self::Output>;

    fn valid_output(o: &Self::Output) -> bool;
}
