// collections can contain multiple values and the data they point to is on the
// heap (can grow or shrink as program runs, rather than being known at compile
// time).
use std::collections::HashMap;

fn main() {
    // needs type annotation since we haven't inserted any items yet
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    // iteration over mutable references
    for i in &mut v1 {
        *i *= 2; // dereference then modify
    }
    // iteration over immutable references
    for i in &v1 {
        println!("{}", i);
    }

    let v2 = vec![1, 2, 3]; // vec! macro used when you have initial values

    // Can access vector elements using indexing notation or the get method.
    // These work a bit differently, since indexing either retrieves the
    // element or panics (if it doesn't exist), whereas get returns an
    // Option<&T> which can be processed using match
    let _third = &v1[2];
    match v2.get(2) {
        Some(e3) => println!("v2.get(2) is {}", e3),
        _ => ()
    }

    // Since we can't have mutable and immutable references within the same
    // scope, we actually can't modify the vector in a scope where there's an
    // immutable reference
    {
        let mut _vtmp = vec![1, 2, 3];
        let _first = &_vtmp[0];
        // _vtmp.push(4); // error due to immutable borrow on previous line
    }

    // when we need to store elements of different types in a vector, we can
    // use enums to help us out (the variants of an enum are defined under the
    // same enum type, even though they can hold entirely different data). If
    // we don't know the types until runtime, that's where trait objects can
    // help us out
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(9.81),
        SpreadsheetCell::Text(String::from("Hello, world!")),
    ];

    // strings are implemented as a collection of bytes, and some methods that
    // provide useful functionality when those bytes are interpreted as text.
    // Indexing into a string is complicated by how String data may be
    // interpreted. The str type is built into the language while the String
    // type is provided by the standard library (both are UTF-8 encoded).
    let mut s = "hello".to_string(); // any type implementing Display trait
    // let mut s = String::from("hello"); // equivalent to the above

    s.push(','); // single character
    s.push_str(" world!"); // grow string

    let s1 = "foo".to_string();
    let s2 = "bar".to_string();
    // let s = s1 + &s2; // invalidates s1 since + op takes ownership of 1st
    let s = format!("{}{}", s1, s2); // concatenates without taking ownership

    // Rust strings don't support indexing (i.e., s[0] won't give you the first
    // character of the String s). This has to do with String being a wrapper
    // over a Vec<u8>, and some Unicode values requiring 2 bytes of storage.
    // Also, indexing a String can't provide O(1) lookup in Rust since we'd
    // have to walk through the vector to determine how many valid elements
    // are present. However, if we tell Rust specifically that we want a string
    // slice (containing specific bytes), we can do that
    let ss = &s[0..=3];
    println!("{}", ss);

    // We can also iterate over Strings pretty easily
    for c in s.chars() {
        println!("{}", c);
    }
    // ... and their byte representations
    for b in s.bytes() {
        // valid Unicode scalars may be more than 1 byte
        println!("{}", b);
    }

    // HashMaps allow us to look up data (in a table of sorts) using a key
    // which can be of any type. However, like vectors, all the keys must be
    // the same type for the same HashMap instance. Each key can only have one
    // value associated with it at a given time
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Green"), 2);
    scores1.insert(String::from("Red"), 4);
    // Unlike vectors and strings, HashMaps don't have a lot of support from
    // the standard library where initialization is concerned. Another way to
    // populate a HashMap, besides the above, is to use iterators and the
    // collect method on a vector of tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let values = vec![10, 50];
    // Need the type annotation here since collect() could result in many
    // different data structures. The underscores are used to allow the data
    // types parametrizing the hash map to be inferred.
    let mut scores2: HashMap<_, _> =
        teams.into_iter().zip(values.into_iter()).collect();
    scores2.insert(String::from("Blue"), 20); // overwriting a value
    // We can also insert a value conditionally, if it does not exist. The
    // entry function returns a mutable reference to the value for the
    // corresponding key, if it exists. Such a mutable reference could be used
    // to update values in the dictionary directly, in code following the call
    // to the entry method
    scores2.entry(String::from("Blue")).or_insert(30);
    scores2.entry(String::from("Red")).or_insert(22);
    // Types implementing Copy are copied directly into the HashMap, while
    // owned values will be moved (transferring ownership to the HashMap)
    if let Some(val) = scores2.get(&String::from("Blue")) {
        println!("val is {}", val);
    }
    // iteration
    for (key, val) in &scores2 {
        println!("{}: {}", key, val);
    }
}
