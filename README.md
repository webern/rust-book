
## Notes from The Rust Book

This repository is for a group that is studying *The Rust Programming Language* by Klabnik and Nichols.

## Discussion about Cargo.toml and `tokio`

There are two types of targets in Rust:
- `lib.rs`: people will build our library from source.
- `main.rs`: people will compile a binary and run it.

Note: when we compile a `lib.rs`, it is basically just to make sure it can be compiled.
There is effectively no concept of consuming a library as a binary in Rust. `*`

`*`: well, you can link to Rust binaries, but it's rare. Usually for C interop.

- `lib.rs`: "pinning" a version in Cargo.toml is bad.

Cargo.toml
- version = "1.21.1"
- This means the minimum version we allow.
- These versions could be chosen: 1.21.2, 1.22.3, 1.9999.9999

For lib.rs
I have found a bug that started in 1.21.
I want to pin my `main.rs` build to 1.21.

This is better for `lib.rs`
- version = "1"

## `main.rs`

Now I really am in control.

# In the Real World

## AWS SDK for Rust
- vX.Y.Z
- X is a major version UNLESS it is 0.
- If X is 0, then Y is the major version.
- 0.49.0 to 0.50.0 is MAJOR version bump.
- 0.49.0 to 0.49.1 is a MINOR version bump (not a patch).

Another issue with AWS Rust SDK. Inter-lib dependency.
These must all have a matching major version due to the libraries' design.
- aws-x 
- aws-y
- aws-ec2

AWS SDK is version ZERO.

## coldsnap

Both the lib.rs and main.rs are in the same package.
 
## tuftool

It would technically be OK to pin, but seems unnecessary.
