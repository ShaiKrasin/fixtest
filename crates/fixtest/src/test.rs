use std::{error::Error, panic};

pub enum Failure {
    Err(Box<dyn Error>),
    Panic(&'static str),
}

pub enum Outcome {
    Passed,
    Failed(Failure),
}

type TestFn = fn() -> Result<(), Box<dyn Error>>;

pub struct Test {
    name: &'static str,
    func: TestFn,
}

impl Test {
    pub const fn new(name: &'static str, func: TestFn) -> Self {
        Self { name, func }
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) fn run(&self) -> Outcome {
        match panic::catch_unwind(|| (self.func)()) {
            Ok(Ok(())) => Outcome::Passed,
            Ok(Err(err)) => Outcome::Failed(Failure::Err(err)),
            Err(err) => {
                let message: &'static str = err.downcast_ref::<&'static str>().unwrap_or(&"");
                Outcome::Failed(Failure::Panic(message))
            }
        }
    }
}

// Note that `test` is the module that defines the `Test` struct,
// while `test::tests` is the module for unit tests of the `test` module.
#[cfg(test)]
mod tests;
