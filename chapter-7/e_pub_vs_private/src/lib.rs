#![allow(dead_code, unused_variables, unused_mut)]

/// From p 117, "child" code can use private code from its "parent" but not the other way around
mod a {
    fn private() {}

    // fn nope() {
    //     // This is not allowed. The "parent" cannot use the "child's" private symbols.
    //     crate::a::b::private();
    // }

    mod b {
        fn private() {
            // This is allowed. The "child" can use the "parent's" private symbols.
            crate::a::private()
        }
    }
}

/// p 123 Idiomatic function namespacing
pub mod utils {
    pub mod frobincators {
        pub fn frobnicate() {}
    }
}

pub mod do_this {
    use crate::utils::frobincators;

    fn some_function() {
        frobincators::frobnicate()
    }
}

pub mod not_this {
    use crate::utils::frobincators::frobnicate;

    fn some_function() {
        frobnicate()
    }
}

pub mod some_module {
    pub struct ClashingName;
}

/// Renaming imports
mod rename_a_symbol {
    use crate::some_module::ClashingName as Renamed;

    fn some_function() {
        let _ = Renamed;
    }
}

pub mod some_trait {
    pub trait ClashingTrait {
        fn foo(&self) {}
    }
}

struct MyFoo;

impl some_trait::ClashingTrait for MyFoo {}

mod rename_a_trait {
    use crate::MyFoo;
    // sometimes you just need to bring a trait into a scope and don't care about the name. If the
    // name clashes with something, you can do this.
    //
    // Also note that the trait cannot be used without this.
    use crate::some_trait::ClashingTrait as _;

    fn use_clashing_name_trait() {
        let x = MyFoo;
        x.foo();
    }
}
