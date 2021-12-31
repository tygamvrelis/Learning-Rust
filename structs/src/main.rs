// structs are used to keep associated pieces of data connected with each other
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

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block
impl Rectangle {
    // 4. Final version. Note that Rust performs automatic referencing for the
    // caller (e.g., we write rect.area() instead of (&rect).area())
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated functions don't need to have self as the first parameter if
    // they're not intended to be class methods. Can use such a technique to
    // define (effectively) static class methods
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 1. The code below is BAD. The width and height parameters are related (same
// rectangle), but this is not necessarily clear from the function signature
fn _compute_area1(width: u32, height: u32) -> u32 {
    width * height
}

// 2. The code below is better, but still not ideal. It's better because it
// suggests that the two values passed in are related (by virtue of them being
// grouped together), but now the method implementation becomes a bit confusing
// since it's not obvious what dims.0 and dims.1 are (i.e., would prefer more
// descriptive names)
fn _compute_area2(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}

// 3. The code below is better, but again, not ideal. This is because it's very
// specific to the rectangle type now and won't work with any other type. Our
// final version of this function turns it into a method
fn _compute_area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
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
    let _foo = UnitStruct;

    let rect1 = Rectangle {
        width: 2,
        height: 3,
    };
    let rect2 = Rectangle {
        width: 1,
        height: 1,
    };
    println!("rect1 is {:#?}", rect1);
    // dbg!(...) prints file and line number along with the expression, then
    // returns ownership of the expression
    dbg!(&rect1);
    println!("Area is {}", rect1.area());
    println!("rect1 can hold rect2 is {}", rect1.can_hold(&rect2));
    let sq1 = Rectangle::square(1);
    println!("sq1 is {:#?}", sq1);
}
