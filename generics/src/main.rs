// Generics are a tool to help us handle the duplication of concepts for
// different types. They express behaviour in terms of abstractions rather than
// concretions. Generics can apply to functions, structs and enums. Traits are
// another tool in Rust that allow us to define behaviour in a generic way;
// they constrain generic types to only those with a particular behaviour.
// Lifetimes are a variety of generics that provide the compiler with info
// about how references relate to each other (help write more flexible code
// while still allowing the compiler to ensure reference validity). Rust
// compiles generic code into code that uses concrete types, so we pay no
// runtime cost for using generics.

// When we use a generic parameter in the body of a function, we also need to
// declare the parameter name in the function signature so the compiler knows
// how to interpret the name. find_max<T>(...) is interpreted as "the function
// find_max is generic over some type T". In this particular function, the
// algorithm needs to perform elementwise comparisons, which means we must be
// able to order the elements. Not all types T will be comparable in such a
// way, so to restrict this function to only types which can be ordered, we use
// a trait
fn find_max<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            // T needs to have PartialOrd trait
            max = item; // T needs to have Copy trait
        }
    }
    max
}

struct Point<T> {
    x: T,
    y: T,
}

// Function available for all generic types T. Note that here, type T
// parametrizes the impl block, while it is possible for the methods within the
// block to have their own distinct parameters
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Function available *only* for f32 type
impl Point<f32> {
    fn dist_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let num_list = vec![2, -3, 42, 0, 16];
    let max = find_max(&num_list);
    println!("Max of {:?} is {}", num_list, max);

    let char_list = vec!['h', 'e', 'l', 'l', 'o'];
    println!("Max of {:?} is {}", char_list, find_max(&char_list));

    let int_struct = Point { x: 2, y: -2 };
    let float_struct = Point { x: 2.12, y: -6.93 };
    println!("int_struct.x is {}", int_struct.x());
    println!(
        "float_struct point is {} units away from origin",
        float_struct.dist_from_origin()
    );
}
