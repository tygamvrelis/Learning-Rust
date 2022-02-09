// Threads are suitable when multiple parts of your code can run independently
// or simultaneously. Some classic challenges with multithreading are:
// - race conditions, where resources are accessed in inconsistent order
// - deadlocks, where multiple threads are stuck in a state where they are
//   waiting for each other's resources to be released
// In general, bugs can arise in very specific situations that arise
// infrequently and can be difficult to reproduce (e.g., classic priority
// inversion problem). Rust attempts to mitigate the possible dangers
// associated with using threads. The Rust standard library only provides an
// implementation of 1:1 threading, meaning each Rust thread corresponds
// precisely to 1 OS thread; this is in contrast to languages providing a green
// threading (M:N) model, at the cost of a larger amount of runtime code.

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("{} from spawned thread", i);
            // sleep to allow a different thread to run
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=5 {
        println!("{} from main thread", i);
        // sleep to allow a different thread to run
        thread::sleep(Duration::from_millis(1));
    }
    // new thread will be stopped when the main thread ends, which may occur
    // before the new thread's task is done. We can choose to wait for the
    // new thread to finish
    handle.join().unwrap();

    // We can use move closures to transfer ownership of values from the main
    // thread to a spawned thread. The move keyword forces the closure to take
    // ownership of the values it's using in the environment. If we don't do
    // this, then Rust will infer that it should borrow v, but it won't let us
    // do that since it can't guarantee the reference to v will always be valid
    // (since it doesn't know how long the thread will run for). It's important
    // to note that the move keyword doesn't violate Rust's ownership rules
    // (they are still enforced); rather, it just lets us override Rust's
    // conservative default of borrowing
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("{:?} from spawned thread", v);
    });
    handle.join().unwrap();
}
