//! # My Documentation Crate
//!
//! This is a crate I'm using to learn about Cargo and Crates.io
//!
//! It contains some simple functions, such as `add_one`!

/// Adds one to the given number
///
/// # Examples
///
/// ```
/// assert_eq!(6, documentation::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}