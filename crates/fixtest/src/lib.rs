#![warn(clippy::pedantic)]

mod runner;
mod test;

pub use runner::main;
pub use test::Test;
