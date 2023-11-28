/*!

# Chapter 8: Collections

pp. 131-149 in the 2018 edition paperback.

To try the Rustlings exercises:

| Exercise               | Book Chapter        |
| ---------------------- | ------------------- |
| 05_vecs                | §8.1                |
| 09_strings             | §8.2                |
| 11_hashmaps            | §8.3                |

```shell
git clone git@github.com:webern/rust-book.git
cd rust-book
cd chapter-8/rustlings
cargo run --package rustlings-chapter-8 -- watch
```

 */
#![allow(dead_code, unused_variables, unused_mut)]

use anyhow::{Context, Result};
use std::any::Any;
use std::collections::HashMap;

fn main() {
    println!("Chapter 8!");
    vectors().unwrap();
    multiple_types_in_vectors();
    typical_string_operations().unwrap();
    hash_maps().unwrap();
}

/// https://doc.rust-lang.org/std/collections/index.html
fn all_collections() {}

/// **Vectors**
fn vectors() -> Result<()> {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);

    // Avoid accessing elements in a vector this way when possible. This will panic.
    // let does_not_exist = &v[100];

    // Tiny performance hit but safer. Returns a `Option::None` when the element does not exist.
    let does_not_exist = v.get(100);

    ////////////////////////////////////

    // When you take an immutable reference to an element in the vector, it makes the vector
    // immutable and prevents borrowing other elements mutably.
    //
    // This is an immutable reference to the first value.
    let first = v.first().context("Vector is empty")?;

    // This is a mutable reference to the second value. It will not compile becuase there is an
    // immutable reference held to another element.
    // let second = v.get_mut(1).context("Missing second element")?;

    // This also will not compile because pushing could cause the internals to move the element to
    // which we are holding a mutable reference. i.e. the entire vector is immutable.
    // v.push(7);

    println!("The first element is {}", first);

    Ok(())
}

/// Vectors can only hold a single type, i.e. all elements are an `i32` or all elements are a
/// `String`. The book shows how you can use an Enum to hold multiple types in a vector.
fn vector_of_enums() {
    enum TwoTypes {
        I(i32),
        S(String),
    }

    // Does not compile.
    // let no_mixed_vecs = vec![0, String::from("Hello")];

    // Does compile.
    let enum_vec = vec![TwoTypes::I(0), TwoTypes::S(String::from("Hello"))];
}

/// The standard library provides a way to hide types. Sean used this in the Bottlerocket settings
/// SDK. This is advanced, we will go over some of these concepts later, but this is like holding
/// a list of Java `Object` pointers or Go `interface{}` pointers that need to be downcast.
fn multiple_types_in_vectors() {
    let mut vec = Vec::new();
    let zero = 0;
    vec.push(Box::new(zero) as Box<dyn Any>);
    vec.push(Box::new(String::from("Hello")) as Box<dyn Any>);
    vec.push(Box::new(0u128) as Box<dyn Any>);

    let item_id = (&*vec.get(1).unwrap()).type_id();

    for item in &vec {
        if let Some(value) = item.downcast_ref::<String>() {
            println!("The trimmed String is {}", value.trim_end().trim_start());
        } else if let Some(value) = item.downcast_ref::<i32>() {
            println!("The value is one less than {}", value + 1)
        } else {
            let type_id = (&*item).type_id();
            println!("Unable to downcast unexpected type '{:?}'", type_id)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn typical_string_operations() -> Result<()> {
    // You can append to the end of a string in multiple ways.
    let mut mut_string = String::from("Hello");
    // Push a character.
    mut_string.push(',');
    mut_string.push(' ');
    // Push a str.
    mut_string.push_str("World");
    assert_eq!("Hello, World", mut_string);

    // You can also use + to concatenate, which forms a new String.
    let hello_world = mut_string + "!";
    assert_eq!("Hello, World!", hello_world);

    // You can use the format! macro in two ways.
    let foo = "foo";
    let bar = "bar";
    // Here we use the variable names in the braces.
    let format_1 = format!("{foo} {bar}");
    // Here we use empty braces and add the variables as arguments.
    let format_2 = format!("{} {}", foo, bar);
    assert_eq!("foo bar", format_1);
    assert_eq!("foo bar", format_2);

    ////////////////////////////////

    // You cannot index into a string. Why? Because Strings are UTF-8 but internally the data is
    // a Vec of bytes. Some UTF-8 characters take one byte, some take multiple bytes, so indexing
    // into the bytes is not allowed (does not compile).
    // let cannot_index = &format_1[0];

    // We can get an iterator over the characters from a String and use that.
    let c = format_1.chars().skip(1).next().unwrap();
    assert_eq!('o', c);

    // String slices. Though you cannot index into a String, you *can* slice them. I find this
    // surprising. It happens at the byte level, which means if you do not slice on the UTF-8
    // boundaries, the slice attempt will panic. Bad! Don't do this unless you are absolutely sure
    // you cannot hit a multibyte UTF-8 character in the wrong place.
    let utf_8 = String::from("¿Qué?");
    // Panics: byte index 1 is not a char boundary; it is inside '¿' (bytes 0..2) of `¿Qué?`'
    // let bad = &utf_8[1..4];

    // This is what we meant above:
    let qué = &utf_8[2..6];
    assert_eq!(qué, "Qué");

    // This is a better, safer way... use an iterator over the UTF-8 characters.
    let safer_qué = utf_8.chars().skip(1).take(3).collect::<String>();
    assert_eq!(safer_qué, "Qué");

    Ok(())
}

/// https://doc.rust-lang.org/book/ch08-03-hash-maps.html
fn hash_maps() -> Result<()> {
    // Creating empty:
    let mut hash_map = HashMap::new();

    // Adding Key value Pairs
    hash_map.insert("One", 1);
    hash_map.insert("Two", 2);

    // Appending with an iterator
    let multiple_items = vec![("Three", 3), ("Four", 4)];
    hash_map.extend(multiple_items);

    // Adding a key only if not present:
    let five = hash_map.entry("Five").or_insert(5);
    assert_eq!(*five, 5);
    // This does *not* update the value because the key already exists:
    let still_five = hash_map.entry("Five").or_insert(98765);
    assert_eq!(*still_five, 5);

    // Note: the above function, `entry(key)` returns a mutable reference, so we can use it for the
    // a canonical programming interview-type question, counting words. From the book:
    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        // also works
        // TODO - check this to be sure
        for word in text.split_whitespace() {
            let count = map.get(word).cloned().unwrap_or_default();
            map.insert(word, count + 1);
        }
    }

    Ok(())
}

// TODO - maybe move this to the session on lifetimes.
/// This question came up during at the end.
///
/// Q: Why is it that sometimes you can get away with passing a ref to a temporary to a function but
///    other times you can't.
/// A: When the compiler knows that the lifetime of the referenced object only needs to be as long
///    as the function, i.e. your temp does not need to outlive the function's scope, the compiler
///    allows this.
mod ref_to_temp {
    fn returns_an_owned() -> String {
        String::from("blah")
    }

    fn takes_and_returns_ref(x: &str) -> &str {
        x.trim_end()
    }

    /// This is allowed because nobody ever tries to use the variable `slice`, so the compiler knows
    /// that the referenced object only needs to live as long as the `takes_and_returns_ref`
    /// function body.
    fn this_works() {
        let slice = takes_and_returns_ref(&returns_an_owned());
    }

    /// This is not allowed because the compiler sees that `slice` is used later which means the
    /// temp object returned by `returns_an_owned` needs to live longer than the
    /// `takes_and_returns_ref` function body.
    fn this_does_not_compile() {
        let slice = takes_and_returns_ref(&returns_an_owned());
        // Uncomment this and compilation will fail.
        // println!("{slice}");
    }

    /// This is how you fix `does_not_compile`. You need to assign the result of `returns_an_owned`
    /// to a variable that lives long enough.
    fn this_fixes_the_problem() {
        let s = returns_an_owned();
        let slice = takes_and_returns_ref(&s);
        println!("{slice}");
    }
}
