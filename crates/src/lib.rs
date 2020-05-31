//! # My Crate
//!
//! `my_create` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = crates::add_one(arg);
///
/// assert_eq?(6, answer);
/// ```
///
/// # Panic
///
/// # Errors
///
/// # Safety

pub fn add_one(x: u32) -> u32 {
    x + 1
}
