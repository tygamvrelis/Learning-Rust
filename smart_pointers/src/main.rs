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

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Hello, world!");
}
