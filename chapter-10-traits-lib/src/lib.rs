#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

pub trait Foo {
    fn foo(&self) -> String;
}

pub struct NotMyStruct {
    inner: i32,
}
