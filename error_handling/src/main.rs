// Rust distinguishes between recoverable and unrecoverable errors. The first
// type of error might be something like a file not existing, in which case we
// may wish to just create the file instead, while the second type of error is
// a bug, such as attempting to access an array element outside the bounds.
// Setting the RUST_BACKTRACE env var to 1 will show us all the functions
// called to get to the point of error; the topmost line that *we* wrote will
// be the one in error. Also, we need to compile with debug symbols (i.e., not
// release mode) in order to view the backtrace.
use std::fs::File;
use std::io::{self, ErrorKind};

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
    let f = match f {
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
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn main() {
    loop {
        println!("Enter a number between 1 and 4");
        println!("\t1 => demo panic");
        println!("\t2 => demo out of bounds heap access");
        println!("\t3 => file open demo");
        println!("\t4 => file open demo 2");
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
            _ => {
                println!("Unexpected value! Try again");
                continue;
            }
        }
    }
}
