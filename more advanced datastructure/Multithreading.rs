Using threads to run cdoe simultaneously
Using message passsing to share data between threads


what is multithreading
multithreading is a programming concept that allows multiple threads to run concurrently within a single process. Each thread represents a separate path of execution, enabling tasks to be performed simultaneously or in an interleaved manner. This can lead to improved performance and responsiveness, especially in applications that require parallel processing or handle multiple tasks at once.   \
Threads in Rust
In Rust, multithreading is achieved using the standard library's `std::thread` module
, which provides tools for creating and managing threads. Rust's ownership and type system help ensure memory safety and prevent data


trying to run multiple things in a one time is called parallelism



use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}