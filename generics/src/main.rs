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

// Just like how Rust can often infer types, it can also infer the lifetimes of
// references (i.e., the scope within which a given ref is valid). Sometimes we
// have to explicitly tell Rust types when multiple are possible, and
// similarly, sometimes we have to annotate relationships using generic
// lifetime params to ensure that refs are valid at runtime. The Rust compiler
// has a borrow checker to helper accomplish this. The borrow checker uses the
// size of the scope of a variable to determine the lifetime for which it is
// valid; it then checks all borrows (references), and compares whether the
// lifetime of the value holding the reference exceeds the lifetime of the
// value it is referring to. If such a case is detected, the compiler throws an
// error.
// The function below has a complication from a lifetime perspective;
// basically, the borrow checker can't determine whether the reference returned
// will always be valid, since it can't know ahead of time whether the if or
// else block will execute (and hence whether the value is borrowed from s1 or
// s2). To fix this, we need to add lifetime annotations.
// Lifetime annotations help describe the relationships of lifetimes among many
// references without actually affecting the lifetimes. A lifetime annotation
// for a single parameter doesn't carry much meaning, since these annotations
// are meant to convey relationships. For example, if two references passed
// into a function both have a general lifetime 'a, it means that both of the
// references must live at least as long as the generic lifetime. In that case,
// the generic lifetime parameter 'a will end up being replaced with the
// concrete lifetime that is the shorter of the two parameters. Although this
// can restrict usage of the function, it ensures that results will always be
// used in a valid manner (i.e., lifetime of result can't exceed lifetime of
// its borrowed value).
// Lifetime parameter names always begin with an apostrophe ', and 'a is the
// go-to in the same way that T is the go-to for generic types.
// In summary, lifetime annotations effectively describe constraints we want
// Rust's borrow checker to enforce (i.e., don't compile if any scenarios can
// arise that violate the constraints). Lifetimes don't need to be annotated
// within a function, just to or from code outside a function. Finally, a
// function which returns a reference must have a lifetime parameter for the
// return type that matches the lifetime parameter for one of the function
// parameters (if it did not, then the returned type must refer to a reference
// created within the function, which would be a dangling reference after going
// out of scope at the end of the function).
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// Lifetime param 'a here indicates that an instance of this struct can't
// outlive the reference held in the part field. If there were more fields that
// were references, we would need to provide a lifetime annotation for each of
// them too. In this case, the lifetime param is considered part of the
// struct's type
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn tst(&self) {
        println!("ImportantExcerpt: {}", self.part);
    }
}

// The Rust compiler recognizes certain patterns of references; in these cases,
// you don't have to write the lifetime parameters explicitly. The patterns are
// called lifetime elision rules, and may grow as Rust development continues.
// Lifetimes on function parameters are called input lifetimes, and on return
// values are called output lifetimes. When there aren't explicit lifetime
// params, the compiler uses 3 rules to try determining the lifetimes
// references have:
// 1. Each parameter gets its own lifetime reference
// 2. If there is just 1 input lifetime param, that same one is assigned to all
//    output ones
// 3. If there are multiple input lifetime params, but one of them is &self or
//    &mut self, then the lifetime of self is applied to all output lifetime
//    params
// These rules can often help us write cleaner code. For example, class methods
// look much cleaner when there's not an explicit lifetime param for &self

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

    let string1 = String::from("test");
    {
        let string2 = String::from("longer_test");
        // Based on the lifetime annotations for longest, res will get the
        // shorter of the two lifetimes for the two arguments. In this case,
        // res's lifetime will be equal to that of string2, which is the inner
        // scope
        let res = longest(string1.as_str(), string2.as_str());
        println!("1: Longest string is {}", res);
    }
    // The code below will result in a compile error due to the value that res
    // borrows not living long enough. It doesn't matter whether string2 or
    // string1 is longer; what matters is the lifetimes of the arguments and
    // whether the *function* could possibly return either of them.
    // let res;
    // {
    //     let string2 = String::from("l");
    //     res = longest(string1.as_str(), string2.as_str());
    // }
    // println!("Longest string is {}", res);
    // Unlike the above case, the below works because string2 is a static
    // string (i.e., a literal) and is therefore valid for the entire duration
    // of the program since it's (baked into the binary)
    let res;
    {
        // let string2 = "longer_test"; // short form
        let string2: &'static str = "longer_test"; // full form
        res = longest(string1.as_str(), string2);
    }
    println!("2: Longest string is {}", res);

    let ex = ImportantExcerpt {
        part: &string1,
    };
    ex.tst();
}
