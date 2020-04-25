// only library creates expose functions that other crates can use;
// binary crates are meant to be run their own.
// this is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs
// file that calls that lives in the src/lib.rs, in that structure we can use integration to test lib

// cargo will compile each of the files as an individual crate
// cargo test --test integration_test
// use brings things from a path to the current scope, mostly brings a module from a crate
use adder::*;
// mod declares a module
mod common;

#[test]
fn test_add_two() {
    common::setup();
    assert_eq!(4, add_two(2));
}
