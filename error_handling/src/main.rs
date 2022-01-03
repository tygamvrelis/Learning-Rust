// Rust distinguishes between recoverable and unrecoverable errors. The first
// type of error might be something like a file not existing, in which case we
// may wish to just create the file instead, while the second type of error is
// a bug, such as attempting to access an array element outside the bounds.
// Setting the RUST_BACKTRACE env var to 1 will show us all the functions
// called to get to the point of error; the topmost line that *we* wrote will
// be the one in error. Also, we need to compile with debug symbols (i.e., not
// release mode) in order to view the backtrace.

// Result<T, E> should be used when you want calling code to decide how to
// handle errors. panic! should be used when there's no way to recover (e.g.,
// your code could end up in a bad state). For example, panicking when the
// calling contract of a function is violated (i.e., the inputs violate the
// requirements of the function), is a good idea since it's invalid usage by
// the caller. Such contracts for a function, especially when violations will
// cause a panic, should be explained in API documentation

// To ensure that, e.g., a variable, enforces a particular contract (say, this
// value should be an int between 1 and 20), we can create a custom type where
// the "contractual" obligations are checked in the constructor. This way, the
// instance is only successfully created if the obligations are all satisfied.
// Otherwise, we can make sure the constructor panics, causing the calling code
// to fail. Such a case would indicate to the caller programmer that they need
// to perform some input validation before creating this instance. For the rest
// of the following code, the programmer can rest assured that if the instance
// was created, then it is a valid instance. Such a design would also allow
// this contract to be portable, in the sense that we don't have to worry about
// making these checks manually every time (although the caller will need to
// worry about this)
use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn demo_panic() {
    // Unrecoverable errors are dealt with using panic!
    panic!("Crash and burn");
}

fn demo_out_of_bounds() {
    // Will also cause a panic
    let v = vec![1, 2, 3];
    v[3]; // out of bounds. Runtime error since vec is on heap
}

fn demo_file_open() {
    let f = File::open("hello.txt"); // Result<File, std::io::Error>
    // All of this nested matching can be implemented more cleanly using
    // closures and unwrap_or_else, but we'll get to those later
    match f {
        // If it opens properly, then f is assigned file
        Ok(file) => file,
        // If there's an error opening the file, we take different actions
        // based on the error type
        Err(error) => match error.kind() {
            // If the file was not found, we try to create it
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            // If it's a different error, we consider it unrecoverable
            other_error => {
                panic!("Problem opening file {:?}", other_error);
            }
        },
    };
}

fn demo_file_open2() {
    // The Result type has helper methods to make the code in demo_file_open
    // more concise via shortcuts. For example, the unwrap() method will get
    // the value inside the Ok variant if it's a match, otherwise it'll panic.
    // The expect macro basically does the same thing, but allows us to specify
    // the panic message
    let _f = File::open("hello.txt").unwrap();
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn demo_err_prop() -> Result<String, io::Error> {
    // Propagating errors to the caller is a good idea when we don't
    // necessarily know how the errors should be handled (i.e., it's more
    // appropriate for the caller to decide what to do).
    // Error propagation is a common pattern in Rust, and the ? operator is
    // intended to make this easier. Basically, an expression ending in ? will
    // cause an Err to be returned from the function if there's a problem,
    // otherwise if everything's okay, it'll evaluate to the value inside Ok.
    // Upon error, the ? operator results in the from function being called,
    // which converts the error type to the one that the function is expected
    // to return (if needed). Naturally, this requires the error type to
    // implement the from function in order to control this conversion. The ?
    // operator can only (I think...) be used in functions that have a return
    // type of Result<T, E> or Option<T> or another type implementing
    // std::ops::Try
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    // File::open("hello.txt")?.read_to_string(&mut s)?; // alternative chain
    Ok(s)
}

fn main() {
    loop {
        println!("Enter a number between 1 and 4");
        println!("\t1 => demo panic");
        println!("\t2 => demo out of bounds heap access");
        println!("\t3 => file open demo");
        println!("\t4 => file open demo 2");
        println!("\t5 => demo error prop");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => {
                demo_panic();
                break;
            }
            2 => {
                demo_out_of_bounds();
                break;
            }
            3 => {
                demo_file_open();
                break;
            }
            4 => {
                demo_file_open2();
                break;
            }
            5 => {
                demo_err_prop().expect("Error opening file!");
                break;
            }
            _ => {
                println!("Unexpected value! Try again");
                continue;
            }
        }
    }
}
