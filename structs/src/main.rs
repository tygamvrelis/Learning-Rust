#[derive(Debug)]
struct User {
    active: bool,
    // not &str, because we want the struct to own all its data. If we want the
    // struct to contain references, it requires the use of *lifetimes*, which
    // are a specific Rust feature
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Scope 1");
    {
        let user1 = build_user(String::from("example@mail.com"), String::from("user1"));
        // struct update syntax. In this case, the reference to user1 is
        // invalidated by the update, since ownership of the username string is
        // moved
        let _user2 = User {
            email: String::from("example2@mail.com"),
            ..user1 // fields not explicitly set should have same value as user1
        };
        // the line below results in a compiler error due to the partially
        // moved value of user1
        // println!("user1: {:#?}\nuser2: {:#?}", user1, user2);
    }
    println!("Scope 2");
    {
        let user1 = build_user(String::from("example@mail.com"), String::from("user1"));
        // struct update syntax. In this case, the reference to user1 is
        // invalidated by the update, since ownership of the username string is
        // moved
        let user2 = User {
            email: String::from("example2@mail.com"),
            username: String::from("user2"),
            ..user1 // fields not explicitly set should have same value as user1
        };
        println!("user1: {:#?}\nuser2: {:#?}", user1, user2);
    }

    // tuple structs are useful when you want to add meaning to a collection of
    // related values, but don't necessarily need names associated with the
    // fields
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    let _e1 = black.1; // access element at index 1 same as for a tuple

    // unit structs behave similarly to the unit type, (), and can be useful
    // when we need to implement a trait on some type but don't need any data
    // to be stored in the type itself
    struct UnitStruct;
    let foo = UnitStruct;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
