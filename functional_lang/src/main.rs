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

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // Needed for iterator trait, "associated type"

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
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

#[test]
fn iterator_demo() {
    let v1 = vec![4, 5, 6];

    // calling next() changes internal state, therefore need mutable
    let mut v1_iter = v1.iter(); // iterator over immutable references
                                 // can call into_iter() to get val ownership
                                 // can call iter_mut() to get mutable refs

    assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), Some(&5));
    assert_eq!(v1_iter.next(), Some(&6));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let total: i32 = v1.iter().sum(); // sum is a "consuming adaptor"
    assert_eq!(total, 6);
}

#[test]
fn iterator_adaptor() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn iterator_trait_methods_on_counter() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| 2 * b - a)
        .filter(|x| x % 2 == 0)
        .sum();
    assert_eq!(sum, 10);
}