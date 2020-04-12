// Errors are a fact of life in software
// Rust groups errors into two major categories: recoverable and unrecoverable
// a file not found error is recoverable and it's reasonable to report the problem to user
// unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array
// Rust has the type Result<T, E> for recoverable errors and
// panic! macro for an unrecoverable error to stops execution

mod panic;

fn main() {
    panic::main();
}
