// enums are possible when we want a value to come from a custom set of
// variants (i.e., a list of possibilities). Sometimes we will want to
// associate an enum value with a data type; this could be accomplished using a
// struct, but a better way to do this in Rust is to put the data directly into
// each enum variant (i.e., each variant can be associated with a different
// type and amount of data). Furthermore, the name of each enum variant becomes
// a function that constructs an instance of the enum. The flexibility of this
// enum system allows us to effectively define several related structs:
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// the above is similar to defining separate structs for each type of message,
// but now we have the benefit of seeing that they're clearly related, and we
// can build abstractions for the Message type rather than needing to handle
// each variant of it separately. We can also define methods on enums:
impl Message {
    fn call(&self) {
        dbg!(self);
    }
}

fn main() {
    let msg = Message::Write(String::from("Hello, world!"));
    msg.call();

    // A useful enum provided by the standard library is Option, which lets a value
    // either be something, or nothing. This is similar to null in other languages,
    // but importantly, this is captured by Rust's type system instead of just
    // being a particular value. Rust doesn't have nulls, but this Option enum
    // encodes the concept of a value being present or absent
    let absent_number: Option<i32> = None;
    // The compiler won't let us use an Option<T> as if it is definitely a
    // valid value. Instead, we have to convert an Option<T> to a T before
    // using it as a T. This avoids one of the most common issues with null:
    // assuming that something isn't null when it actually is
    let y: Option<i8> = Some(5);
    let x: i8 = 16;
    // let sum = x + y; // errors since i8 and Option<i8> can't be added
}
