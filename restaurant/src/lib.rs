// Modules allow us to group related functionality together and name why
// they're related. To find an item in a module tree, we use a naming path,
// similarly to how we navigate a filesystem. Two types:
//    - Absolute path: starts from the crate root by using the crate name or
//      a literal usage of "crate"
//    - Relative path: uses an identifier in the current module, or self or
//      super. super is similar to .. in a file system
// Paths separate identifiers by double colons, ::
// The choice of whether to use absolute or relative paths (and similarly,
// whether to use super), depends on how you expect the relationships between
// dependent parts of your code to change (or not). For example, if two parts
// of your code are always expected to remain in the same relative positions in
// the hierarchy, then using relative paths with super:: would be convenient.
// Modules also define Rust's privacy boundary. By default, all items are
// private.
mod front_of_house; // semicolon after mod name says to load it

mod back_of_house;

// This statement makes hosting a valid name in the crate root. The idiomatic
// way to bring functions into scope is to bring the function's parent module
// into scope, so that the parent module will still have to be specified when
// calling the function. This makes it clear that the function is not locally-
// defined. We can re-export a name that we bring into scope by prefixing it
// with the pub keyword (in this case, external code could then call the
// add_to_waitlist function by calling hosting::add_to_waitlist()). pub use can
// be useful when the internal structure of the code is different from how
// programmers might think about the domain. We can write our code with one
// structure in mind, then expose a different structure to whoever is consuming
// the library. This separates the concerns of how to think about the library's
// internals from its public interface.
pub use crate::front_of_house::hosting; // absolute
// use self::front_of_house::hosting; // relative

// For data structures, the idiom is to bring them fully into scope. The
// exception is when we are bringing two items with the same name into the same
// scope (which is not allowed, anyway). In this case, we might choose to
// bring the parent module into scope, or create a new local name for the type
// using the as keyword.
use std::collections::HashMap;
// use std::collections::*; // glob operator brings in everything
use std::fmt::Result as FmtResult; // type alias

// Nested paths allow us to bring in many different items into scope using
// fewer lines of code
use std::{cmp::Ordering, io::Read};

// Another example below brings std::io into scope, along with one of its
// children.
// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// eat_at_restaurant and front_of_house are siblings; thus, eat_at_restaurant
// can access public modules within front_of_house, even though front_of_house
// isn't pub.
pub fn eat_at_restaurant() {
    // The statements below are needed when we don't have the use statement to
    // bring the crate::front_of_house::hosting module into the scope of
    // eat_at_restaurant.
    // crate::front_of_house::hosting::add_to_waitlist(); // absolute
    // front_of_house::hosting::add_to_waitlist(); // relative
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("White");
    meal.toast = String::from("Whole wheat"); // can modify pub toast field
    // meal.seasonal_fruit = String::from("strawberries"); // can't modify

    // This is getting further and further away from a restaurant function...
    let mut map = HashMap::new();
    map.insert(1, 2);
}
