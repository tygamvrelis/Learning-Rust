use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

// Main function should delegate its tasks to functions so that it becomes more
// clear what the responsibilities of the program are and how they are
// separated.
// Would be nice if all error-handling logic was in one place so that future
// maintainers of the code only had to consult one place if it needed to change.
// Errors should be descriptive so that the user can identify the problems more
// easily.
fn main() {
    // Collect turns the args env::args() iterator into a collection. It can
    // create a variety of collections, so we need to explicitly annotate the
    // type we desire (in this case, a Vec<String>)
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Argument parsing problem: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
