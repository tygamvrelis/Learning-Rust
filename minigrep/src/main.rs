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
    // pass ownership of the iterator returned by env::args
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Argument parsing problem: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
