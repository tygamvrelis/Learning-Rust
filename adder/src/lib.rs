// Testing:
// 1. Set up state
// 2. Run code under test
// 3. Assert the results are expected
// A test in Rust is a function annotated with the test attribute, i.e.,
// #[test]. An attribute is metadata about a piece of code.
// For assert_eq! and assert_ne!, the values being compared need to implement
// the PartialEq and Debug traits. Usually these can just be derived (i.e.,
// default implementation).
// Any arguments passed into the assert macros, beyond their required ones, are
// passed into format!. This is useful because it allows us to print custom,
// details debugging messages when assertions fail.
//
// There are lots of options for changing the behaviour of cargo test, see them
// by running "cargo test --help". The test binary also has its own set of
// options, which can be viewed by running "cargo test -- --help".
//
// For example, tests run concurrently by default, so you have to make sure
// they don't have shared state (test isolation is a best practise anyway).
// However, it is possible to specify only 1 thread if you wish to run tests
// consecutively.
//
// As another example, tests only show messages from println! if they fail. We
// can change this setting by running "cargo test -- --show-output".
//
// We can run a specific test by name, e.g., "cargo test it_works". We can also
// run a subset of tests by specifying a substring contained within their
// names, e.g., "cargo test it" will run the it_works and it_adds_two tests.
//
// For time-consuming tests that we don't want to run everytime, we can
// annotate them with the #[ignore] attribute. If we want to run all ignored
// tests, we can use the command "cargo test -- --ignored"

// There are two categories of tests we think about in Rust: unit and
// integration tests.
// Unit tests lie in the src directory, in the same file as the code they're
// testing. Conventionally, they lie in a module named test that's annotated
// with #[cfg(test)], which tells Rust to compile and run that code only when
// you run "cargo test".
// Integration tests are external to a library and use your library the same
// way other code would (i.e., they use its public API). The purpose is to
// test whether the various parts of your library work together properly.
// Integration tests require a tests directory next to src; each file in this
// directory is compiled as an individual crate. Files in the tests directory
// are only compiled when we run "cargo test". We can still run a specific
// integration test by specifying its name (or a substring), and can run a
// specific set of integration tests by specifying the file name (e.g., "cargo
// test --test integration_test"). We may wish to include some common helper
// code to set up the integration tests, and to avoid this getting its own line
// in the test output, we can put this code in tests/common/mod.rs, since files
// in subdirectories of tests don't get compiled as separate crates or have
// sections in the test output. Note that integration testing only applies to
// library crates; binary crates with a src/main.rs file are meant to be run on
// their own, and offload most of the functionality that lives in libraries.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2 // change to x + 3 to see custom assertion error message
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // alternatively: use crate::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 2,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        let larger = Rectangle {
            width: 10,
            height: 2,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        let input = 2;
        let expected = 4;

        let res = add_two(2);

        assert_eq!(
            expected, res,
            "Did not add two! Inputted {} and got {}",
            input, res
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")] // substring of expected message
    fn guess_greater_than_100() {
        Guess::new(110);
    }

    // Writing tests that return a Result<T, E> allows us to use the ?
    // operator in the body of tests. This might be useful when we want to run
    // a few operations in our test and fail if any of them return an Err
    // variant
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 != 5 {
            Ok(())
        } else {
            Err(String::from("two plus two equals five"))
        }
    }
}
