use super::{Failure, Outcome, Test};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("dummy error")]
struct DummyError;

#[test]
fn test_passes_if_func_returns_ok() {
    let test = Test::new("passing test", || Ok(()));
    assert!(
        matches!(test.run(), Outcome::Passed),
        "test was expected to pass, but didn't"
    );
}

#[test]
fn test_fails_if_func_returns_err() {
    let test = Test::new("failing test", || Err(Box::new(DummyError)));
    assert!(
        matches!(
            test.run(),
            Outcome::Failed(Failure::Err(err)) if err.is::<DummyError>()
        ),
        "test was expected to fail with error, but didn't"
    );
}

#[test]
fn test_fails_if_func_panics() {
    let test = Test::new("panicking test", || panic!("oops"));
    assert!(
        matches!(test.run(), Outcome::Failed(Failure::Panic(message)) if message == "oops"),
        "test was expected to fail by panic, but didn't"
    );
}
