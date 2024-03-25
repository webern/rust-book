/*!

# Chapter 17: Object-Oriented Features

(No Rustlings Exercises for Chapter 17)

 */
#![allow(dead_code, unused_variables, unused_mut)]

use anyhow::{ensure, Result};

fn main() {
    println!("Chapter 17: Object-Oriented Features!");
    sep();
}

/// # Object-Oriented Concepts
///
/// ## Gang of Four
///
/// If you haven't read *Design Patterns: Elements of Reusable Object-Oriented Software* by Erich
/// Gamma, Richard Helm, Ralph Johnson, and John Vlissides...
///
/// Please Do! The concepts are incredibly useful building blocks for maintainable code.
/// https://a.co/d/6otZGno
///
/// ## Objects Contain Data and Behavior
///
/// In object-oriented programing, an *Object* packages data and functions together. Rust fits this
/// description with `structs` (and `enums`) and `impl` blocks where `self` is used.
///
/// ## Encapsulation that Hides Implementation Details
///
/// Another object-oriented concept is that objects can hide implementation details. In Rust we do
/// this by making struct data members (and some functions) private. For example, here is a deeply
/// encapsulated object.
pub struct Encapsulated {
    /// This data is private and thus an "implementation detail", it cannot be accessed by users.
    private_data: i32,
}

impl Encapsulated {
    /// This serves as a constructor because a user would have no way to instantiate the struct.
    pub fn new(value: i32) -> Result<Self> {
        // We use a private function to validate the data.
        Self::validate(value)?;
        Ok(Self {
            private_data: value,
        })
    }

    /// The only way to access the value is through a public function.
    pub fn value(&self) -> i32 {
        self.private_data
    }

    /// The only way to change the value is through a public function and thus, we are able to
    /// validate the incoming value, and we can ensure that the struct is always in a valid state.
    pub fn set_value(&mut self, value: i32) -> Result<()> {
        Self::validate(value)?;
        self.private_data = value;
        Ok(())
    }

    /// This function is private and is thus an implementation detail.
    fn validate(value: i32) -> Result<()> {
        ensure!(
            value > -1,
            "The value needs to be greater than -1, received '{}'",
            value
        );
        Ok(())
    }
}

/// Here is an example of an un-encapsulated object. This object is problematic because users will
/// rely on the internal structure, which means you can never change it without a breaking change.
pub struct NotEncapsulated {
    /// Users are going to rely on the type and name of this field, you can never change it.
    pub public_data: i32,
}

/// # Inheritance
///
/// Rust kind of does have inheritance, in a way. You can't inherit a struct's data (i.e. fields),
/// but you can inherit behavior through the mechanism of default trait implementations. Strictly
/// speaking, this does not mean that Rust supports inheritance like other OO languages do.
pub trait HasImplementation {
    /// This trait member has a default implementation.
    fn say_hello() {
        println!("Hello")
    }
}

/// I can "inherit" the behavior of the trait be choosing not to override the default
/// implementation.
struct HelloBot;

// I don't have to do anything here, I get the trait's default implementation by "inheritance".
impl HasImplementation for HelloBot {}

/// # Polymorphism
///
/// https://en.wikipedia.org/wiki/Polymorphism_(computer_science)
///
/// ## Ad-hoc Polymorphism
///
/// This means using the same function name but allowing it to take different arguments. This is not
/// supported in Rust. By extension, default argument values are not supported either.
///
/// ```java
/// class AdHocPolymorphic {
///     public String add(int x, int y) {
///         return "Sum: "+(x+y);
///     }
///
///     public String add(String name) {
///         return "Added "+name;
///     }
/// }
/// ```
///
/// ## Parametric Polymorphism (i.e. Generics)
///
/// Rust does support this:
///
/// ```rust
/// struct<T> Whatever(T);
/// ```
///
/// ## Subtyping
///
/// If a class derives from a superclass, then it can be substituted for the superclass per the
/// Liskov Substitution Principle. Rust does *not* support this. You can `impl` multiple traits on a
/// struct (or enum), but they are not hierarchical, i.e. one can not extend another. Basically you
/// cannot do this is Rust:
///
/// ```java
/// abstract class Pet {
///     abstract String speak();
/// }
///
/// class Cat extends Pet {
///     String speak() {
///         return "Meow!";
///     }
/// }
///
/// class Dog extends Pet {
///     String speak() {
///         return "Woof!";
///     }
/// }
///
/// static void letsHear(final Pet pet) {
///     println(pet.speak());
/// }
///
/// static void main(String[] args) {
///     letsHear(new Cat());
///     letsHear(new Dog());
/// }
/// ```
fn polymorphism() {}

// TODO: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
// TODO: https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html

/// Used to separate printed things.
fn sep() {
    println!();
    println!("----------------------------------------------------------------------------------");
    println!();
}

// A question came up during the session about whether traits are more like inheritance or
// interfaces. (I think that was the question.) I didn't quite know how to answer but provided this
// example. This comes from a Java example in the polymophism Wikipedia, and shows how Rust can do
// either compile-time or runtime resolution of traits.

trait Pet {
    fn live(&self);
}

struct Cat;

impl Pet for Cat {
    fn live(&self) {
        todo!()
    }
}

struct Dog;

impl Pet for Dog {
    fn live(&self) {
        todo!()
    }
}

/// In this example, the compiler generates code at compile-time that has zero cost. For each
/// different T that is passed, a different version of this function is generated (generics).
fn compile_time_trait_usage<T: Pet>(pet: T) {
    todo!()
}

/// In this example, only one version of this function exists, and its code finds the right `Pet`
/// function(s) through dynamic dispatch at runtime (which has a runtime cost).
fn runtime_trait_usage_dynamic_dispatch(pet: &Box<dyn Pet>) {
    todo!()
}

fn use_the_above_functions() {
    let cat = Cat;
    let dog: Box<dyn Pet> = Box::new(Dog);
    compile_time_trait_usage(cat);
    runtime_trait_usage_dynamic_dispatch(&dog);
}

fn holding_a_vector_of_pets_requires_dynamic_dispatch() {
    let pets: Vec<Box<dyn Pet>> = vec![Box::new(Cat), Box::new(Dog)];
}
