fn main() {
    // Ownership is a unique feature of Rust which relates to data on the heap
    // Some rules:
    //     1. Each value in Rust has a variable that’s called its owner.
    //     2. There can only be one owner at a time.
    //     3. When the owner goes out of scope, the value will be dropped.
    // For example, string literals (e.g., "hello") are immutable. However, a
    // String can be modified. If we want to create a String from a literal, we
    // can use String::from("testX")
    println!("Scope 1");
    {
        let mut s = String::from("test1"); // requests memory from the
                                           // allocator at runtime
        s.push_str(", yo!"); // s is mutable
        println!("\t{}", s);
    } // s goes out of scope, automatically returns the memory to the allocator
      // via the drop function (automatically called at the closing bracket)
    println!("Scope 2");
    {
        let s1 = String::from("test2");
        let s2 = s1; // s1 is moved to s2
                     // unlike a shallow copy, this assignment invalidates s1
                     // println!("{}, world!", s1); // Causes a compilation error, because
                     // Rust doesn't allow us to use an
                     // invalidated reference
        println!("\t{}, yo!", s2);
    }
    println!("Scope 3");
    {
        let s1 = String::from("test3");
        let s2 = s1.clone(); // deeply copies the heap data. s1 and s2 valid
        println!("\ts1 = {}, s2 = {}", s1, s2);
    }
    // Data types in Rust might implement either the Drop or Copy trait
    // (mutually exclusive). The Drop trait is used when the type requires
    // something special to happen when the value goes out of scope. The Copy
    // trait is used when we want an older variable to still be usable after
    // assignment
    println!("Scope 4");
    {
        let s = String::from("test4");
        // s moves into the function and is invalidated
        takes_ownership(s); // can't use s after this
                            // println!("{}", s); // causes an error

        let s = String::from("test4");
        let s = takes_and_gives_back_ownership(s); // gets ownership back
        println!("\t{}", s);

        // x is an i32, which has the Copy trait, so it's okay to use x again
        // in this scope after makes_copy returns
        let x = 5;
        makes_copy(x);
    }
    // Having to return anything passed into a function in order to regain
    // ownership is tedious. Could manage this by returning tuples, or better
    // yet, by passing in arguments as *references*. References allow you to
    // refer to a value without taking ownership of it. Creating a reference is
    // called "borrowing". By default, we are not allowed to modify something
    // we have a reference to (i.e., are borrowing). Also, we cannot borrow
    // something more than once at a time (this prevents data races at compile
    // time). Furthermore, we cannot have a mutable reference while we also
    // have an immutable one. However, we can have multiple immutable
    // references simultaneously. Some rules:
    //     1. At any given time, you can have either one mutable reference or
    //        any number of immutable references
    //     2. References must always be valid
    println!("Scope 5");
    {
        let s = String::from("test5");
        uses_reference(&s);
        println!("\t{}", s);

        let mut s = String::from("test5");
        // let r1 = &mut s; // would cause compilation error, if we used r1
        change_by_reference(&mut s);
        println!("\t{}", s);

        // A reference's scope starts when it is introduced and ends at its
        // last usage. The Rust compiler guarantees that data will not go out
        // of scope before all references to it do
        let r1 = &s;
        let r2 = &s;
        println!("\tr1 = {}, r2 = {}", r1, r2); // r1 and r2 not used after this

        let r3 = &mut s;
        println!("\tr3 = {}", r3);
    }
}

fn takes_ownership(ss: String) {
    println!("\ttakes_ownership: {}", ss);
}

fn takes_and_gives_back_ownership(ss: String) -> String {
    println!("\ttakes_and_gives_back_ownership: {}", ss);
    ss // returned and moved to the caller
}

fn makes_copy(xx: i32) {
    println!("\tmakes_copy: {}", xx);
}

fn uses_reference(ss: &String) {
    println!("\tuses_refernce: {}", ss);
    // ss.push_str("content"); // causes an error
}

fn change_by_reference(ss: &mut String) {
    ss.push_str("content"); // causes an error
    println!("\tchange_by_reference: {}", ss);
}
