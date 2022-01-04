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
// a trait. A different way to specify that a function takes any type that
// implements traits X and Y would be to write that argument in the form of,
// e.g., "list: &(impl X + Y)". What this really means is shown in its full
// form below.
// An alternate syntax option involves using the "where" clause, e.g.,
// fn some_func<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// { ... }
// In summary, traits let us specify to the compiler circumstances under which
// a generic type must have a certain set of behaviours.
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

// Conditionally implements the cmp_display method, with this depending on the
// traits that the type T implements. Using a similar technique, we can write
// impl blocks for all generic types T which implement a set of traits (called
// trait bounds); this is called a blanket implementation. An example is the
// to_string() method, which is implemented for all types T which have the
// Display trait
impl<T: std::fmt::Display + PartialOrd> Point<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x >+ y");
        } else {
            println!("x < y");
        }
    }
}

// Function available *only* for f32 type
impl Point<f32> {
    fn dist_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Traits define shared behaviour across types in an abstract way. By
// behaviour, we mean the methods we can call on that type. Traits are similar
// to *interfaces* in other languages, although they're not exactly the same.
// They can be thought of as a way to group method signatures (not the actual
// implementations; that is handled separately by each type which implements
// the trait) that are needed to accomplish some particular purpose. If we want
// another crate to be able to implement our trait for its own types, then we
// need to declare it as pub.
pub trait Summary {
    // We can force the trait-implementer to provide their own implementation
    // by simply declaring the method, i.e.:
    fn summarize_author(&self) -> String;

    // This is a default implementation of the trait method
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Article {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Calling code doesn't know the concrete type that will be returned; has to
// rely on the interface, so to speak. However, using this "impl trait" syntax
// for the return type has the restriction that only one concrete type can be
// returned (i.e., can't sometimes return an Article and other times return a
// Tweet).
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("hunter2"),
        content: String::from("Hello, world!"),
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

    let tweet = returns_summarizable();
    println!("Tweet: {}", tweet.summarize());

    let article = Article {
        author: String::from("hunter2"),
        headline: String::from("Hello, world!"),
        content: String::from("LOREM IPSUM!!"),
    };
    println!("Article: {}", article.summarize());
}
