/*!

# Chapter 16: Concurrency

To try the Rustlings exercises:

| Exercise               | Book Chapter        |
| ---------------------- | ------------------- |
| threads                | §16.1-3             |
| smart_pointers         | §15, §16.3          |

```shell
git clone git@github.com:webern/rust-book.git
cd rust-book
cd chapter-16/rustlings
cargo run --package rustlings-chapter-16 -- watch
```

 */
#![allow(dead_code, unused_variables, unused_mut)]

use std::ops::{Deref, DerefMut};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Note: this entire chapter says nothing about `async`, `await`. The chapter was probably written
/// before those keywords were introduced. It is less likely that you will use `std::thread` for
/// concurrency these days, and more likely that you will use things like `Stream`s from the
/// `futures` or `tokio` libraries.
fn main() {
    println!("Chapter 16!");
    sep();
    simple_threads_with_join();
    sep();
    moving_things_into_threads();
    sep();
    channels();
    sep();
    mutexes();
}

/// With a `thread` we can run code in-parallel to the main thread.
///
/// `thread::spawn` returns a `JoinHandle`,
/// https://doc.rust-lang.org/std/thread/struct.JoinHandle.html, which we can use to return
/// something from the thread.
///
fn simple_threads_with_join() {
    println!("Simple threads with join:");

    // We use `thead::spawn` to start a thread.
    let handle = thread::spawn(|| {
        // Inside the thread we have a loop.
        for i in 1..=8 {
            println!("spawned thread: {}", i);
            // Sleep to simulate computation or IO.
            thread::sleep(Duration::from_millis(1));
        }
        // Return the result of some expensive calculation.
        "Hello World!"
    });

    // The main thread has a loop running in parallel.
    for i in 1..5 {
        println!("*---main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Pretend our thread actually did something and get a result from it.
    let expensive_result = handle.join().unwrap();
    println!("expensive result: {}", expensive_result)
}

/// In general, you need to move ownership of a thing into a thread. You cannot pass a reference
/// into a thread because the compiler doesn't understand that lifetime implications of something
/// like the `JoinHandle::join()` function.
fn moving_things_into_threads() {
    println!("Moving things into threads:");
    // For example, I cannot pass a reference to this string into the thread.
    let string = String::from("*");

    // I won't be able to pass this into the thread.
    let ref_str = string.as_str();

    // Since I have a ref above, I'm going to pass this clone into the thread.
    let string_clone = string.clone();

    let handle = thread::spawn(move || {
        // This is fine, I have moved ownership of the cloned string into this thread.
        println!("Here's the cloned string: {}", string_clone);

        // DOES NOT COMPILE: The compiler doesn't understand the lifetime of the reference.
        // println!("Here's a reference to the original string: {}", ref_str);
    });

    handle.join().unwrap();
    // Technically, you and I understand that the string variable outlives the thread, but the
    // compiler doesn't understand this. The re-joining of a thread is ot a primitive.
    println!("Here's the original string: {}", string);
}

/// Rust has channels if that's your thing. Go touts channels as the best thing since sliced bread,
/// so if you are coming from Go, you should be familiar with these. Channels allow you to send a
/// "message" (i.e. a value of some sort) from one thread to another.
///
/// For an example of deep channel usage in Bottlerocket, checkout apiserver exec witten by Tom:
///
/// https://github.com/bottlerocket-os/bottlerocket/blob/f6c1b/sources/api/apiclient/src/exec.rs#L84
fn channels() {
    println!("Channels:");
    // By convention, the sender is tx (transmitter?) and the receiver is rx.
    let (tx, rx) = mpsc::channel();
    // You can have more than one sender:
    let tx2 = tx.clone();

    // Thread 1
    let handle_1 = thread::spawn(move || {
        for i in 1..=5 {
            tx.send(format!("thread 1: {}", i)).unwrap();
            thread::sleep(Duration::from_micros(1000));
        }
    });

    // Thread 2
    let handle_2 = thread::spawn(move || {
        for i in 1..=5 {
            tx2.send(format!("thread 2: {}", i)).unwrap();
            thread::sleep(Duration::from_micros(600));
        }
    });

    for received in rx {
        println!("Message received: {}", received);
    }

    // Be a good citizen and join your threads.
    handle_1.join().unwrap();
    handle_2.join().unwrap();
}

/// The evil scary mutexes (Go has made mutex a dirty word).
///
/// Obtain a lock in one thread and other threads are blocked from obtaining a lock until the
/// locking thread drops its lock.
///
/// Note that the `Mutex` is immutable, but it offers a mutable reference to what it holds. This is
/// the "interior mutability" pattern in Rust. The other object typically used for this is `RefCell`
/// which does not offer the locking protection.
///
fn mutexes() {
    println!("Mutexes:");
    // It is typical to see Arc<Mutex<T>>.
    let data = Arc::new(Mutex::new((String::from("empty"), 0u32)));
    // If we want more than one thread to have access, we can clone the mutex by way of Arc.
    let clone_1 = data.clone();
    let clone_2 = data.clone();

    let handle_1 = thread::spawn(move || {
        for _ in 1..=10 {
            let mut lock = clone_1.lock().unwrap();
            let (message, counter) = lock.deref_mut();
            *counter += 1;
            *message = format!("Thread 1 updated the counter to {}", counter);
            // Release the lock before sleeping!
            drop(lock);
            thread::sleep(Duration::from_micros(100));
        }
    });

    let handle_2 = thread::spawn(move || {
        for _ in 1..=10 {
            let mut lock = clone_2.lock().unwrap();
            let (message, counter) = lock.deref_mut();
            *counter += 1;
            *message = format!("Thread 2 updated the counter to {}", counter);
            // Release the lock before sleeping!
            drop(lock);
            thread::sleep(Duration::from_micros(67));
        }
    });

    let handle_3 = thread::spawn(move || {
        loop {
            let mut lock = data.lock().unwrap();
            let (message, counter) = lock.deref();
            if *counter == 20 {
                println!("Counter reached 20, threads are done with their loops!");
                break;
            }
            println!("{}", message);
            // Release the lock before sleeping!
            drop(lock);
            thread::sleep(Duration::from_micros(10));
        }
    });

    handle_1.join().unwrap();
    handle_2.join().unwrap();
    handle_3.join().unwrap();
}

/// The `Sync` and `Send` traits are built-in language markers for types. These traits have no
/// functions, they just serve as information to the compiler whether a type is safe for certain
/// threading conditions. Both are implemented automatically by the compiler when it is determined
/// to be appropriate (e.g. all struct members are `Sync`/`Send`, thus the struct is as well.
///
/// `Sync`: means that it is safe to share a reference to the object between threads. From `Sync`
/// documentation:
/// > - Types that are not Sync are those that have “interior mutability” in a non-thread-safe form,
/// >   such as `Cell` and `RefCell`.
/// > - `&T` is `Send` if and only if `T` is `Sync`
/// > - `&mut T` is Send if and only if `T` is `Send`
/// > - `&T` and `&mut T` are `Sync` if and only if T is `Sync`
///
/// This stuff gets confusing. Maybe we will learn about it if we go through the `Rustnomicon`.
/// For now, know that a type needs to be `Send` in order for you to pass it into a thread.
fn sync_and_send() {}

/// Used to separate printed things.
fn sep() {
    println!();
    println!("----------------------------------------------------------------------------------");
    println!();
}
