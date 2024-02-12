/*!

# Chapter 13: Functional Language Features

To try the Rustlings exercises:

| Exercise               | Book Chapter        |
| ---------------------- | ------------------- |
| iterators              | §13.2-4             |

```shell
git clone git@github.com:webern/rust-book.git
cd rust-book
cd chapter-13/rustlings
cargo run --package rustlings-chapter-13 -- watch
```

 */
#![allow(dead_code, unused_variables, unused_mut)]

use std::thread;
use std::time::Duration;

fn main() {
    println!("Chapter 13!");
    closures();
    sep("Iterators");
    iterators();
}

/// # Closures
///
/// This is an excellent blog post explaining how closures work:
/// <https://rustyyato.github.io/rust/syntactic/sugar/2019/01/17/Closures-Magic-Functions.html>
///
/// **Definition**: Closures are anonymous functions you can save in a variable or pass as arguments
/// to other functions.
/// - Closures can capture variables from the surrounding environment.
/// - Type annotations are usually optional.
fn closures() {
    // Example of a closure that uses type annotation and is assigned to a variable:
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_millis(32));
        num
    };
    // You can then call this like a function:
    let answer = expensive_closure(10);

    // Much of the above syntax is optional. These all do the same thing:
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;
    // The types of add_one_v3 and add_one_v4 are determined by the first call site:
    let _ = add_one_v3(3u32);
    let _ = add_one_v4(3u8);

    // After the first call site, you cannot call the closure with a different type.
    let closure_types = |x| println!("The closure's type has been determined: {}", x);
    closure_types(String::from("String"));
    // Does not compile:
    // closure_types(10);

    closure_immutable_borrow();
    closure_mutable_borrow();
    closure_move_ownership();
}

fn sep(s: &str) {
    let mut line = format!("-- {} --", s);
    for _ in 0..(70 - s.len()) {
        line.push('-')
    }
    println!("\n{}", line);
}

/// # Closures: Capturing References or Moving Ownership: Immutable Reference
///
/// This demonstrates a closure that borrows an immutable references from its surroundings.
fn closure_immutable_borrow() {
    sep("Immutable Borrow Closure");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // The list variable is automatically taken as an immutable reference.
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

/// # Closures: Capturing References or Moving Ownership: Immutable Reference
///
/// This demonstrates a closure that borrows a mutable references from its surroundings.
fn closure_mutable_borrow() {
    sep("Mutable Borrow Closure");
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // Cannot borrow here because a mutable borrow has already been created for the closure!
    // Does not compile:
    // println!("Cannot borrow here!: {:?}", list);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

/// # Closures: Capturing References or Moving Ownership: Immutable Reference
///
/// This demonstrates a closure that takes ownership using the `move` keyword.
fn closure_move_ownership() {
    sep("Move Ownership Closure");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let move_closure = move || println!("From thread: {:?}", list);

    // Cannot use `list` here because it has been moved by the closure.
    // Does not compile:
    // println!("Cannot use list here: {:?}", list);

    thread::spawn(move_closure).join().unwrap();
}

/// There are traits that correspond to types of closures:
///
/// 1. `FnOnce` (`move`): applies to closures that can be called once. All closures implement at
/// least this trait, because all closures can be called. A closure that **moves** captured values
/// out of its body will only implement `FnOnce` and none of the other Fn traits, because it can
/// only be called once.
///
/// 2. `FnMut` (`mut &`) applies to closures that don’t move captured values out of their body, but
/// that might mutate the captured values. These closures can be called more than once.
///
/// 3. `Fn` (`&`) applies to closures that don’t move captured values out of their body and that
/// don’t mutate captured values, as well as closures that capture nothing from their environment.
/// These closures can be called more than once without mutating their environment, which is
/// important in cases such as calling a closure multiple times concurrently.
///
/// Here is an example of `FnOnce` which is used similarly to this for `Option::unwrap_or_default`.
fn unwrap_or_default<T, F>(opt: Option<T>, f: F) -> T
where
    F: FnOnce() -> T,
{
    match opt {
        Some(x) => x,
        None => f(),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// # Iterators
fn iterators() {
    iterator_demonstration();
    the_sum_method_consumes_the_iterator();
    collect_consumes_the_iterator_and_creates_a_vector();
    closures_used_in_iterators_can_also_capture_the_environment();
}

/// # Iterator Trait
/// (Renamed so as not to collide with the language's trait.)
pub trait MyIterator {
    type Item;

    /// This is the only function within the trait that is required to be implemented. All the other
    /// functions have default implementations that use this.
    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

fn the_sum_method_consumes_the_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    // Does not compile:
    // let _ = v1_iter.next();
    assert_eq!(total, 6);
}

fn methods_like_map_and_filter_return_a_new_iterator() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let new_iterator = v1.iter().map(|x| x + 1);
}

fn collect_consumes_the_iterator_and_creates_a_vector() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

fn closures_used_in_iterators_can_also_capture_the_environment() {
    #[derive(Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let shoes_in_size_10 = shoes_in_size(shoes, 10);
    println!("Shoes in size 10:\n{:?}", shoes_in_size_10)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// # Improving Our I/O project
///
/// See: https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html
fn improving_our_io_project() {
    // Look at the book for this one
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// # Comparing Performance: Loops vs. Iterators
///
/// See: https://doc.rust-lang.org/book/ch13-04-performance.html
fn comparing_performance() {
    // Look at the book for this one
}
