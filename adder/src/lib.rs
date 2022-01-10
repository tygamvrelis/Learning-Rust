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

pub struct Guess {
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
