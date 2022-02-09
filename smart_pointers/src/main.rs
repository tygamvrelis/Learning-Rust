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

// The interior mutability pattern in Rust allows you to mutate data even when
// there are immutable references to it. We can do this when we can ensure that
// the borrowing rules will be followed at runtime, even though the compiler
// won't be able to guarantee it. Types using this pattern use unsafe code
// wrapped in a safe API. One example of such a type is RefCell<T>. This type
// is similar to Box<T>, but while misuse of the borrowing rules on a Box<T>
// will cause a compiler error, misuse of a RefCell<T> will cause a runtime
// panic. Checking borrowing rules at compile time has no runtime cost and can
// speed up development, but checking them at runtime allows us to get around
// cases that the compiler can't understand and guarantee. The compiler is
// inherently conservative (would rather reject code than spit out an incorrect
// program), so there are some cases where the programmer can take the
// responsibility upon themselves if they require certain functionality.

// RefCell<T> allows immutable or mutable borrows which are checked at runtime.
// This means we can mutate the value inside it even when it is immutable. This
// idea of mutating the value inside an immutable value is the interior
// mutability pattern. This could be useful for writing a class method which
// takes an immutable reference to self, yet modifies referenced data within
// the class. Such functionality may be required in order to implement specific
// traits. With RefCell<T>, we can use the borrow() and borrow_mut() methods
// (part of the safe API) to get smart pointers of type Ref<T> and RefMut<T>,
// respectively. RefCell<T> keeps track of how many mutable and immutable
// borrows are active, and just like the compile-time rules, it will either
// allow one mutable reference or many immutable references, at any given time
// (and if these rules are violated, we'll get a runtime panic).
// --> key: RefCell<T>'s runtime-checked borrowing rules allow us to make an
//          object that can modify itself in a context where only immutable
//          values are allowed.
pub trait Messenger {
    fn send(&self, msg: &str); // immutable reference to self
}

use std::cell::RefCell;

pub struct MockMessenger{
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![])
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        // Without RefCell implementing the interior mutability pattern, we
        // wouldn't be able to modify the sent_messages vector because this
        // method takes an immutable reference to self
        self.sent_messages.borrow_mut().push(String::from(msg));
    }
}

fn learning_about_refcell() {
    let messenger = MockMessenger::new(); // immutable value
    messenger.send("Hello world!"); // mutates internal state
    assert_eq!(messenger.sent_messages.borrow().len(), 1);
}

// Rust's memory safety makes it difficult, although not impossible, for memory
// to be leaked. This can be done by using Rc<T> and RefCell<T> in cycles, so
// that items refer to each other and thus can never be dropped.

// One way to get around this is to reorganize data structures so that some
// references express ownership and others don't. Strong references are how
// ownership is expressed, since these change reference counts. On the other
// hand, weak references do not express an ownership relation, and will not
// prevent the referenced data from being dropped. A weak reference can be
// obtained by downgrading a strong one, and before doing anything with a
// Weak<T>, we have to check whether the referenced data still exists by
// attempting to upgrade it. We will explore this in the context of a tree data
// structure.

use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    // parent can't be an Rc<Node>, since that would result in a reference
    // cycle (parent -> child -> parent) hence a memory leak. In terms of
    // ownership, a parent should own its children, i.e., if the parent node is
    // dropped, then all its children should be too; however, the reverse is
    // not true, which indicates weak references would be suitable here. We
    // just want a child node to be able to refer to its parent, but don't want
    // it to own its parent
    parent: RefCell<Weak<Node>>,
    // RefCell so that we can modify which nodes are children, and Rc<Node>
    // vector elements so that ownership can be shared with variables
    children: RefCell<Vec<Rc<Node>>>,
}

fn learning_about_ref_cycles() {
    let leaf = Rc::new(Node {
        value: 2,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent is {:?}", leaf.parent.borrow().upgrade());
    // after this line, the leaf node is now owned by (1) leaf and (2) branch
    let branch = Rc::new(Node {
        value: 4,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // assign weak ref
    // lack of infinite output of the below is an indication that this code is
    // free of reference cycles
    println!("leaf parent is {:?}", leaf.parent.borrow().upgrade());
}

fn main() {
    learning_about_box();
    learning_about_mybox();
    learning_about_deref_coercion();
    learning_about_drop();
    learning_about_rc();
    learning_about_refcell();
    learning_about_ref_cycles();
}
