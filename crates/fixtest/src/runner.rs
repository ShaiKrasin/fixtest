use crate::test::{Failure, Outcome, Test};

fn print_test_outcome(test_name: &str, outcome: &Outcome) {
    print!("{test_name}: ");
    match outcome {
        Outcome::Passed => println!("[PASSED]"),
        Outcome::Failed(Failure::Err(ref err)) => println!("[FAILED] {err}"),
        Outcome::Failed(Failure::Panic(message)) => {
            println!("[FAILED] panic!({message:?})");
        }
    }
}

pub fn main(tests: &[&Test]) {
    for &test in tests.iter() {
        print_test_outcome(test.name(), &test.run());
    }
}
