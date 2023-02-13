#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

/// - tests should be isolated. running one test should not affect another test.
/// - tests should not leave state behind. clean up after yourself.
mod isolated {
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn bad_test_1() {
        // BAD: this is not an isolated filepath, someone else might depend on it.
        fs::write("/tmp/foo", "data").expect("Unable to write file");
        assert_eq!(
            fs::read_to_string("/tmp/foo").expect("Unable to read file"),
            "data"
        );
        // BAD: we left the file behind.
    }

    #[test]
    fn bad_test_2() {
        // VERY BAD: using the same non-isolated path as another test.
        fs::write("/tmp/foo", "data2").expect("Unable to write file");
        assert_eq!(
            fs::read_to_string("/tmp/foo").expect("Unable to read file"),
            "data2"
        );
        // BAD: we left the file behind.
    }

    #[test]
    fn good_test() {
        // use TempFile to get a unique directory and to automatically clean up the dir
        let dir = TempDir::new().expect("unable to create tempdir");
        // GOOD: this filepath is unique
        let p = dir.path().join("foo.txt");
        fs::write(&p, "data3").expect("Unable to write file");
        assert_eq!(
            fs::read_to_string(&p).expect("Unable to read file"),
            "data3"
        );
        // GOOD: TempFile automatically deletes the directory.
    }
}

/// Code needed only for tests should not be compiled into the "product" bin/lib.
/// We can use a compile guard over a module if we need functions or constants for tests only.
#[cfg(test)]
mod compile_guarded {
    use std::path::PathBuf;

    const SOME_TEST_ONLY_CONSTANT: &str = "128471";

    fn test_helper() -> PathBuf {
        PathBuf::from(SOME_TEST_ONLY_CONSTANT).join("foo.txt")
    }

    #[test]
    fn a_test() {
        let p = test_helper();
        // do something
    }
}

/// # Cargo New
/// Let's look at a few fun commands
/// ```shell
/// rm -rf /tmp/cargo_new
/// mkdir /tmp/cargo_new
/// cd /tmp/cargo_new
/// cargo new example_bin
/// tree -a example_bin
/// cat example_bin/Cargo.toml
/// cargo new --vcs git --lib --edition 2018 --name package_name the_dir
/// tree -a the_dir
/// ```
mod cargo_new {}

/// Its fine to panic in tests. Since test failures are typically signalled by panicking (that's
/// what the assertion macros do), there is nothing to be gained by avoiding panicking in tests.
mod everybody_panic {
    use tempfile::TempDir;

    #[test]
    fn panic_is_ok() {
        let t = TempDir::new().expect("this panic is fine");
        assert_eq!(
            "this will panic because the values do not match",
            t.path().display().to_string()
        )
    }
}

/// # Doc Tests
///
/// Doc tests are an amazing feature of Rust/Cargo.
///
/// Here we are, writing markdown, and we want to show an example. This example will be compiled
/// and executed during `cargo test`. It passes if it doesn't panic just like normal tests.
///
/// ## Example
///
/// ```
/// use chapter_11a::doc_test_example::MyType;
/// let foo = MyType::new("balloon");
/// assert_eq!("balloon is a thing that I say when I am thinking about balloon", foo.ruminate())
/// ```
pub mod doc_test_example {
    pub struct MyType {
        inner: String,
    }

    impl MyType {
        pub fn new<S: Into<String>>(value: S) -> Self {
            Self {
                inner: value.into(),
            }
        }

        pub fn ruminate(&self) -> String {
            format!(
                "{} is a thing that I say when I am thinking about {}",
                self.inner, self.inner
            )
        }
    }
}

/// # Compile-only Doc Test
///
/// Sometimes your doc test cannot reasonably run. Image a function that returns the first word
/// found on a website. You still want to compile the doc test to make sure it is up-to-date, but
/// you can't rely on the external webpage.
///
/// ## Example
///
/// ```no_run
/// use chapter_11a::compile_only::first_word_on_page;
/// let first_word = first_word_on_page("https://example.com");
/// ```
///
pub mod compile_only {
    pub fn first_word_on_page(url: &str) -> String {
        // Imagine we use `reqwest` to get the first word of a web page and return it.
        // I can prove that our doc test doesn't run this by panicking.
        panic!("The doc test doesn't actually run this function!")
    }
}

/// # Rant about Test Loops
///
/// Personally I hate tests that loop through a series of inputs. I think this is considered
/// best-practice in Go. I usually won't make you change it in a code review, but here are my
/// reasons:
///
/// - It makes it harder to isolate the test case that failed. When I want to fix the failing test
///   I want to easily run just the failure without commenting out code.
/// - It obscures the actual number of tests. If am testing 10 different inputs, I would like that
///   counted as 10 tests that passed.
///
mod rant_about_loops {
    pub fn trim(s: &str) -> &str {
        // std offers this, but pretend my trim function is complicated
        s.trim()
    }

    #[test]
    fn i_hate_this() {
        let inputs = [" test 1", "test 2 ", " test 3 "];
        let outputs = ["test 1", "test 2", "test 3"];
        for i in 0usize..2 {
            assert_eq!(outputs.get(i).unwrap(), &trim(inputs.get(i).unwrap()));
        }
    }

    // I prefer the more verbose option of creating a test for each test case.

    #[test]
    fn trim_test_1_leading_space() {
        assert_eq!("test 1", trim(" test 1"));
    }

    #[test]
    fn trim_test_2_trailing_space() {
        assert_eq!("test 2", trim("test 2 "));
    }

    #[test]
    fn trim_test_3_leading_and_trailing() {
        assert_eq!("test 3", trim("      test 3     "));
    }
}

/// Be Assertive
/// Demo of the assertion macros and features of the macros.
mod be_assertive {
    fn double(x: i32) -> i32 {
        x + x
    }

    // Don't actually put a bunch of separate tests in one test function. Just showing the macros
    // here...
    #[test]
    fn a_some_assertions() {
        // Assert that two values are equal as long as they implement PartialEq and Display.
        assert_eq!(6, double(3));

        // Assert that two values are different.
        assert_ne!(7, double(3));

        // With all of these, you can add a description. When in doubt:
        // - Check a test failure, see how helpful the panic message is.
        // - If, and only if, the panic message is unclear, add a message.
        assert_eq!(7, double(3), "'double(3)' should return '6'");
    }
}

/// # Should Panic
/// `should_panic` is of limited value unless you are careful.
mod should_panic_tests {
    fn gimme_a_value_greater_than_10(x: u8) -> u8 {
        // CAUTION: this can panic if the subtraction would wrap
        let result = x - 1;
        if x < 10 {
            panic!("'{}' is too high", x);
        }
        result
    }

    // This test passes even though it is panicking for a different reason.
    // This panic says "attempt to subtract with overflow"
    #[test]
    #[should_panic]
    fn unreliable_test() {
        let _ = gimme_a_value_greater_than_10(0);
    }

    // This test fails because we expected it to panic on the range check. Instead it panicked for a
    // different reason and we want to know this.
    #[test]
    // We specify a substring from the expected panic message.
    #[should_panic(expected = "too high")]
    fn reliable_test() {
        let _ = gimme_a_value_greater_than_10(0);
    }
}

/// # Tests Returning Result
///
/// The test system can handle tests that return a `Result`.
mod tests_returning_results {
    use std::fs::read_to_string;

    type Result = std::result::Result<(), anyhow::Error>;

    /// Note: this gives a horrendously unhelpful error message, so maybe don't do this?
    #[test]
    fn returns_a_result() -> Result {
        let s = read_to_string("bad/path/does/not/exit")?;
        assert_eq!("", s);
        Ok(())
    }
}

/// # No Capture
///
/// We don't see the `stdout` and `stderr` from a test unless we tell cargo we want to see it.
///
/// Shows the output (I'm not sure about the difference between these two):
/// `cargo test --package chapter_11 --lib output::example -- --color always --show-output`
/// `cargo test --package chapter_11 --lib output::example -- --color always --nocapture`
///
/// Does not show the output:
/// `cargo test --package chapter_11 --lib output::example`
///
mod output {
    #[test]
    fn example() {
        println!("Hello Nocapture!");
        eprintln!("This is stderr!");
    }
}

// running a subset of tests by name: demo this with jetbrains

/// # Integration Tests
///
/// I disagree with the authors' definition of unit vs. integration tests. Technically they are
/// right, but a more useful delineation, IMO:
/// - `unit tests`: Do not rely on external services (e.g. docker, databases, external web services)
/// - `integration tests`: Require external web services (e.g. launching a containerized instance of
///   postgres and interacting with it, or using AWS services)
///
/// A way to think of it is like this:
/// - It works on any machine? It's a `unit test`.
/// - It requires some setup on the host machine or elsewhere? It's an `integration test`.
///
/// A great way to separate them is to put integration tests behind a cargo feature flag.
/// - Unit tests only: `cargo test --package chapter_11 --lib integ_discussion`
/// - All tests including integ:
///   `cargo test --package chapter_11 --lib integ_discussion --features integ`
///
/// (See the `Cargo.toml` for the feature definition)
///
mod integ_discussion {
    #[test]
    #[cfg(feature = "integ")]
    fn integrate_with_postgres() {
        // launch a docker container running postgres
        // wait for it to come up
        // insert rows
        // check the inserted rows
    }

    #[test]
    fn unit_test() {
        // check some input parsing logic
    }
}

/// # Rant: Write Pure Functions
///
/// Make functions pure instead of dirty and write the damn tests.
///
/// ## This is Almost Always Possible!
///
/// Think about it, do you have side-effects or input happening in the middle of the function?
/// Pull that out of your function and make it pure.
///
mod pure_functions {
    struct Config;

    /// BAD: this is hard to test
    fn read_config_from_etc() -> Config {
        std::fs::read_to_string("/etc/my.conf").unwrap();
        // do the parsing
        Config {}
    }

    /// GOOD: now you can test the function
    fn read_config<P: AsRef<Path>>(path: P) -> Config {
        std::fs::read_to_string(path.as_ref()).unwrap();
        // do the parsing
        Config {}
    }
}
