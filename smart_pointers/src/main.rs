// Smart pointers are data structures that behave like pointers but have
// additional metadata and capabilities. Unlike references, which can only
// borrow data, smart pointers often own the data they point to. Smart pointers
// are typically implemented as structs which implement the Deref and Drop
// traits. There are many existing smart pointers, the most important of which
// are:
// - Box<T> for heap-allocated values
// - Rc<T> for referencing counting to enable multiple ownership
// - Ref<T> and RefMut<T>, accessed through RefCell<T>, which enforces
//   borrowing rules at runtime instead of compile time

// A Box<T> is used to store data on the heap. An example of when this might be
// needed is recursive types, whose space requirements cannot be known at
// compile time (a recursive type can have elements whose type are itself). A
// box just provides a level of indirection and heap allocation; there are no
// other smart pointer capabilities. Since Box<T> implements Deref, we can
// treat boxes like any other references. Since Box<T> implements Drop, it is
// automatically cleaned up, along with the heap data, when an instance goes
// out of scope.
// Example: Cons List (construct function list).
enum List {
    Cons(i32, Box<List>), // store pointer to next list value
    Nil,
}

use crate::List::{Cons, Nil};

fn learning_about_box() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// Learning about the Deref trait: MyBox<T>
// (MyBox<T> will not store data on the heap, since the main purpose of this
// lesson is to learn about the Deref trait)
// Generally speaking, the Deref operator lets us "follow the pointer to the
// data". Basically, it allows us to write code that functions exactly the same
// way whether we pass in ordinary references or a type that implements Deref.
struct MyBox<T>(T); // tuple struct with one element

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T; // associated type for Deref trait (similar to generics)

    fn deref(&self) -> &Self::Target {
        &self.0 // return the element of the tuple
    }
}

fn learning_about_mybox() {
    let x = 5;
    let y = &x; // ref pointing to value of x
    let z = Box::new(x); // box pointing to a copied value of x
    let w = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *w); // *(w.deref())
}

// On a related note, deref coercion is a convenience that Rust performs on
// function and method arguments; it converts a type reference into another
// one. For example, &String to &str is done automatically because String
// implements the Deref trait such that a &str is returned. Deref coercion
// interacts with mutability in some important ways.
fn tst(message: &str) {
    println!("{}!", message);
}

fn learning_about_deref_coercion() {
    let m = MyBox::new(String::from("Hello world!"));
    tst(&m); // Follow deref() calls: &MyBox<String> --> &String --> &str
    tst(&(*m)[..]); // alternative, if there was no deref coercion
}

// The Drop trait lets us customize what happens when a value is about to go
// out of scope, which is especially useful for releasing system resources.
// This is one example of a particular destructor in Rust. Useful in making
// cleanup convenient and safe. Rust's ownership system ensures it is never
// possible to drop a value still in use (references must always be valid, so
// drop can only be called when the value is no longer being used)
struct CustomSP {
    data: String,
}

impl Drop for CustomSP {
    // automatically called when value goes out of scope; not allowed to be
    // manually invoked (would result in a double-free when then automatically
    // called after going out of scope). If we need to force a value to clean
    // up early, we need to use std::mem::drop (included in the prelude)
    fn drop(&mut self) {
        println!("Dropping CustomSP with data `{}`", self.data);
    }
}

fn learning_about_drop() {
    // variables are dropped in the reverse order of their creation
    let _a = CustomSP { data: String::from("I am a") };
    let _b = CustomSP { data: String::from("I am b") };
    let _c = CustomSP { data: String::from("I am c") };
    drop(_b);
    println!("CustomSPs created");
}

// Sometimes a value needs to have multiple owners, e.g., in a graph, a node
// might be owned by all the edges connected to it. To enable this sort of
// multiple ownership, Rust has Rc<T>, which counts the number of references to
// the value to determine whether it is still in use. Rc<T> is used for data on
// the heap that is read by multiple parts of our program, and for which the
// last owner can't be determined at compile time. Rc<T> can only be used in
// single-threaded programs.
enum RcList {
    RcCons(i32, Rc<RcList>), // store pointer to next list value
    RcNil,
}

use crate::RcList::{RcCons, RcNil};

// Rc::clone just increases the reference count; doesn't deepy copy the data
// being referenced. This is why the convention is to use Rc::clone instead of
// a.clone(), because Rc::clone is not a performance hit, while usually the
// clone method would create a deep copy and thus be a performance concern.
use std::rc::Rc;

fn learning_about_rc() {
    let a = Rc::new(RcCons(1, Rc::new(RcCons(2, Rc::new(RcNil)))));
    println!("Ref count: {}", Rc::strong_count(&a));
    let _b = RcCons(3, Rc::clone(&a));
    println!("Ref count after creating _b: {}", Rc::strong_count(&a));
    {
        let _c = RcCons(-3, Rc::clone(&a));
        println!("Ref count after creating _c: {}", Rc::strong_count(&a));
    }
    // The drop implementation for Rc automatically decreases the reference
    // count
    println!("Ref count after _c goes out of scope: {}", Rc::strong_count(&a));
}

fn main() {
    learning_about_box();
    learning_about_mybox();
    learning_about_deref_coercion();
    learning_about_drop();
    learning_about_rc();
}
