/*!

# Chapter 15: XXX

To try the Rustlings exercises:

| Exercise               | Book Chapter        |
| ---------------------- | ------------------- |
| XXXXXXXXX              | §XX.2-4             |

```shell
git clone git@github.com:webern/rust-book.git
cd rust-book
cd chapter-15/rustlings
cargo run --package rustlings-chapter-15 -- watch
```

 */
#![allow(dead_code, unused_variables, unused_mut)]

// TODO - for next time: demonstrate weak pointers
// TODO - for next time:get the rustlings exercises ready

use crate::simple_ptr::SimplePtr;
use std::cell::RefCell;
use std::rc::Rc;

mod simple_ptr;

fn main() {
    println!("Chapter 15!");
    sep();
    simple_ptr();
    sep();
    std_lib_smart_pointers();
    sep();
    box_t();
    sep();
    dynamic_dispatch_with_box();
    sep();
    std_mem_drop();
    sep();
    ref_cell_example();
    sep();
    memory_leak();
}

/// # Smart Pointers
///
/// Smart pointers are data structures that act like a pointer but also have additional metadata and
/// capabilities. Smart pointers allocate memory in a constructor and deallocate that memory in
/// their destructor. In Rust, the destructor is the `Drop` trait. This concept was given the
/// terrible name (in C++, possibly by Bjarne Stroustrup) "Resource Acquisition is Initialization"
/// or (RAII).
///
/// See `simple_ptr.rs` for a simple example.
fn simple_ptr() {
    // Define a struct that we will use our simple pointer to hold a pointer to.
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
        id: u64,
    }

    // This function takes a &Person to demonstrate that our smart pointer can deref.
    fn print_person(person: &Person) {
        println!("{:?}", person)
    }

    // This takes a &mut Person to demonstrate that our smart pointer can deref mut.
    fn change_person(person: &mut Person) {
        person.name += " Surname";
        person.age += 1;
        person.id = 11111;
    }

    // Create a pointer to a heap-allocated Person object.
    let mut simple_ptr = SimplePtr::new(Person {
        name: "Name".to_string(),
        age: 30,
        id: 93856723,
    });

    // Use the smart pointer as if it were a Person reference through Deref and DerefMut.
    print_person(&simple_ptr);
    change_person(&mut simple_ptr);
    print_person(&simple_ptr);
}

/// Here are examples of some of the smart pointers in the standard library.
fn std_lib_smart_pointers() {
    #[derive(Debug)]
    struct CannotClone;
    #[derive(Debug)]
    struct Foo(CannotClone);

    let f = Foo(CannotClone);
    // Does not compile:
    // let f2 = f.clone();
    // println!("{:?}", f);
    // println!("{:?}", f2);
    // let f3 = f;
    // println!("{:?}", f);
    // println!("{:?}", f3);

    // Box<T> for allocating values on the heap
    let boxed = Box::new(0);

    // Rc<T>, a reference counting type that enables multiple ownership
    let rc = Rc::new(0);
    // Compiles! Rc lets us hold multiple pointers to the same heap allocated object:
    let r = Rc::new(Foo(CannotClone));
    let r2 = r.clone();
    println!("{:?}", r);
    println!("{:?}", r2);
    // Does not compile. Clone is implemented but not Copy.
    // let r3 = r;
    // println!("{:?}", r);
    // println!("{:?}", r3);

    // RefCell<T>, Ref<T> and RefMut<T>
    // enforces the borrowing rules at runtime instead of compile time
    // This one is complicated and we will cover it later in the chapter!
    let ref_cell = RefCell::new(String::from("Hello Ref Cell"));
    println!("{}", ref_cell.borrow());
}

/// # `Box<T>`
///
/// A `Box` is nothing but a strongly-owned pointer to something that is allocated in `new` and
/// dropped when the `Box` goes out of scope. Why would we need such a thing? We need it:
/// - It's too big.
/// - When we don't know the size of something at compile time.
/// - When we want to use dynamic dispatch on a trait object.
fn box_t() {}

// ## Infinitely recursive type size
//
// Here is a binary tree `Node` type where the size cannot be calculated at compile time. It does
// not compile. Think about this, how would the compiler figure out the size of this? It is of
// infinite size!
// struct ImpossibleNode {
//     left: Option<ImpossibleNode>,
//     right: Option<ImpossibleNode>,
// }

/// ## Using a Box to fix the Problem
///
/// Now the compiler can figure out the size... It needs space for two pointers (well, two
/// `Option<Box>` objects whose size is always known because pointer sizes are known).
///
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

/// # Using a Box for Dynamic Dispatch
///
/// Here we have a simple trait.
pub trait Speak {
    fn speak(&self);
}

// Below we have two implementations of the Speak trait.

/// Says "Gobble"
struct SayOne;

impl Speak for SayOne {
    fn speak(&self) {
        println!("One");
    }
}

struct SayTwo;

impl Speak for SayTwo {
    fn speak(&self) {
        println!("Two")
    }
}

fn dynamic_dispatch_with_box() {
    let mut vec: Vec<Box<dyn Speak>> = Vec::new();
    vec.push(Box::new(SayOne));
    vec.push(Box::new(SayTwo));
    for item in vec {
        item.speak();
    }
}

// I think we will have talked about the Drop and Deref traits out-of-order, but if not, go
// back to `simple_ptr.rs` to discuss them.
// - https://doc.rust-lang.org/book/ch15-02-deref.html#treating-smart-pointers-like-regular
// - https://doc.rust-lang.org/book/print.html#treating-a-type-like-a-reference-by-implementing-the-deref-trait

/// # std::mem::drop
///
/// https://doc.rust-lang.org/book/print.html#dropping-a-value-early-with-stdmemdrop
///
/// If you want to drop something early, you can not call mything.drop(), instead you have to call
/// std::mem::drop(mything).
///
fn std_mem_drop() {
    println!("Using std::mem::drop");
    let simple_ptr = SimplePtr::new(String::from("std::mem::drop"));
    println!("Dropping simple_ptr early, before the end of the function");
    // Note: take a look at how simple this function is.
    drop(simple_ptr);
    println!("Function is till alive but simple_ptr is dropped");
}

/// # Interior Mutability: A Mutable Borrow to an Immutable Value
///
/// This is a type that lets you work around the borrowing rules with runtime checks.
///
/// Why would you need this?
/// - Multiple threads mutating the same memory (with a mutex or other concurrency protection).
/// -
fn ref_cell_example() {
    struct InteriorMutability {
        vec: RefCell<Vec<String>>,
    }

    impl InteriorMutability {
        fn mutate_an_immutable(&self) {
            self.vec.borrow_mut().push(String::from("1"));
            println!("But I was immutable! {:?}", self.vec)
        }
    }

    let immutable = InteriorMutability {
        vec: RefCell::new(Vec::new()),
    };

    immutable.mutate_an_immutable();
    immutable.mutate_an_immutable();
    immutable.mutate_an_immutable();
}

/// # Reference Cycles Can Leak Memory
///
/// Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create
/// memory that is never cleaned up (known as a memory leak). Preventing memory leaks entirely is
/// not one of Rust’s guarantees, meaning memory leaks are memory safe in Rust. We can see that Rust
/// allows memory leaks by using Rc<T> and RefCell<T>: it’s possible to create references where
/// items refer to each other in a cycle. This creates memory leaks because the reference count of
/// each item in the cycle will never reach 0, and the values will never be dropped.
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            // TODO - Help I don't understand why this is recursive. Oh, I think I see, it's the
            // implementation of `Debug` that causes the infinite recursion.
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

fn memory_leak() {
    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // b has a as its tail
    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        // now a has b as its tail, which has a as its tail, which has b as its tail...
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

/// Used to separate printed things.
fn sep() {
    println!();
    println!("----------------------------------------------------------------------------------");
    println!();
}
