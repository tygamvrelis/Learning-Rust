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

fn basic_threading() {
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

// A popular approach for safe concurrency is message passing: rather than
// communicating by sharing memory, we share memory by communicating. To this
// end, Rust provides a programming concept in the standard library called a
// channel. A channel consists of a transmitter-receiver pair, and is
// considered closed if either of these is dropped.

use std::sync::mpsc; // multiple producer single consumer

fn message_passing() {
    let (tx, rx) = mpsc::channel();

    // clone before moving tx into its spawned thread
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("---> I"),
            String::from("---> Am"),
            String::from("---> The"),
            String::from("---> CLONE"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // spawned thread needs to own tx in order to use it
    thread::spawn(move || {
        let vals = vec![
            String::from("Hello!"),
            String::from("World!"),
            String::from("I am the spawned thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // the ownership rules are vital in writing safe concurrent code, and
        // for this case of message passing, sending a value transfers its
        // ownership to the receiving thread. That is, we cannot use a value
        // after sending it through a channel
    });
    // block until something is received. If we want non-blocking I/O, we can
    // use try_recv() which returns a Result<T, E> with an Ok holding a message
    // if one's available, or an Err if no messages are present at that time
    // Old code:
    //     let received = rx.recv().unwrap();
    //     println!("Got: {}", received);
    for received in rx {
        println!("Got: {}", received);
    }
    // Once the transmitting thread is done sending the vector of strings, the
    // transmitter will go out of scope, which breaks the receive loop
}

// Rust's type system and ownership rules greatly assist in getting shared
// state concurrency correct. Let's look at mutexes in Rust, which are a sync
// primitive that allow threads to access shared memory one at a time. The two
// main rules are:
// 1. You need to try acquiring the lock before using the data it guards
// 2. When you're done using the guarded data, you need to release the lock

use std::sync::{Arc, Mutex};

fn shared_state_concurrency() {
    // single-threaded case
    let m = Mutex::new(5); // must acquire lock before using value, since
                           // Mutex<i32> is a different type than i32
    {
        // m.lock() returns a LockResult which we unwrap below. The wrapped
        // value is a MutexGuard smart pointer, which we can deref to get the
        // data guarded by the mutex. This smart pointer implements the drop
        // trait so that the lock is automatically released once it goes out of
        // scope
        let mut num = m.lock().unwrap();
        *num = 10;
    }
    println!("m = {:?}", m);

    // multi-threaded case
    let cnt = Arc::new(Mutex::new(0)); // allow for multiple owners (atomically)
    let mut handles = vec![];
    for _ in 0..10 {
        // each spawned thread increments the counter by 1 in a safe manner
        let cnt = Arc::clone(&cnt);
        let handle = thread::spawn(move || {
            let mut num = cnt.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final count = {}", *cnt.lock().unwrap());
}

// Send and Sync traits
// If a type implements Send, then it means ownership of such a type can be
// transferred between threads. As a rule of thumb, pretty much all primitives
// are Send except for raw pointers. Types composed entirely of Send types are
// automatically Send too.
// The Sync trait indicates that it is safe for the type to be referenced from
// multiple threads (i.e., an immutable reference to this type is Send).
// Similar composition rules apply.
// To reiterate: if we are creating a new type and we wish for it to implement
// Send and/or Sync, then this is taken care of automatically as long as the
// elements composing the type are also Send and/or Sync. If that is not the
// case, then we will have to implement Send and Sync manually using unsafe
// Rust.

fn main() {
    basic_threading();
    message_passing();
    shared_state_concurrency();
}
