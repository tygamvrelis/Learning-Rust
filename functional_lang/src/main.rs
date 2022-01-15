// Functional programming style often involves using functions as arguments,
// variables and return values. Important part of idiomatic Rust.
// Rust provides support for this way of thinking. Closures are function-like
// things that can be stored in variables and iterators are for processing
// collections. Pattern matching and enums are influenced by this way of
// thinking too.
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

// memoization/lazy evaluation pattern can be used alongside closures. Structs,
// enums and functions that use closures are annotated with generics and trait
// bounds.
struct Cacher<T, U, V>
where
    U: std::hash::Hash + std::cmp::Eq,
    T: Fn(U) -> V, // the generic T is an anonymous func w/this signature
{
    calculation: T,
    value_map: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    U: std::hash::Hash + std::cmp::Eq + Copy,
    V: Copy,
    T: Fn(U) -> V,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            value_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.value_map.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value_map.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // variable stores an anonymous function. Compiler can infer param and
    // return types, but only one concrete type can be associated with each of
    // these.
    // Closures can capture values from the scope they're defined in. Based on
    // what the closure body does this these values, we may need to add FnMut
    // or FnOnce trait bounds.
    let num_secs = 2;
    let mut expensive_closure = Cacher::new(|num| { // |<param1>, ..., <paramN>| { <body> };
        println!("Calculating...");
        thread::sleep(Duration::from_secs(num_secs));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

fn main() {
    generate_workout(24, 7);
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn check_old_value_remains_present() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let _v2 = c.value(2);
    let v1 = c.value(1);

    assert_eq!(v1, 1);
}

#[test]
fn call_with_str() {
    let mut c = Cacher::new(|a: &str| a.len());

    let string1 = String::from("Hello world!");
    let v1 = c.value(&string1[..]);

    assert_eq!(v1, 12);
}
