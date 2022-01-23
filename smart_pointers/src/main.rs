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

fn main() {
    learning_about_box();
    learning_about_mybox();
    learning_about_deref_coercion();
}
