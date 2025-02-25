#[derive(Debug)]
pub enum ProgResult<Output> {
    Out { output: Output, steps: usize },
    Error,
    Timeout,
}

pub trait ProgGenerator<Program> {
    fn next<'a>(&'a mut self) -> &'a Program;

    fn register_result<O>(&mut self, program: &Program, result: &ProgResult<O>);
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
