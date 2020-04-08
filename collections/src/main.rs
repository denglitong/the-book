// unlike array and tuple types, the data Rust collections points to is stored on the heap,
// that means the amount of data does not need to be known at compile time and
// can grow or shrink as the program runs.
// - vector: store a variable number of values next to each other, vector can only store values of the same type
// - string: a collection of characters
// - hash map: associate a value with a particular key
mod exercise;
mod hashmap;
mod string;
mod vector;

fn main() {
    vector::main();
    string::main();
    hashmap::main();
    exercise::main();
}
