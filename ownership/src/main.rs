// Rust's central feature is ownership.
// All programs have to manage the way they use a computer's memory while running.
// Rust's memory is managed through a system of ownership with a set of rules that
// the compiler checks at compile time. None of the ownership features slow down your program at running.
// Stack vs Heap
// All data stored on the stack must have a known, fixed size.
// Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
// Managing heap data is why ownership exists. Keeping track of what parts of code are using what data on the heap,
// minimizing the amount of duplicate data on the heap, can cleaning up unused data on the heap are all problems
// that ownership addresses.
// 1. Each value in Rust has a variable that's called its owner.
// 2. There can only be one owner at a time（CPU同一时刻只有一个线程在真正执行，如果是多核并行也会涉及互斥锁）
// 3. When the owner goes out of scope, the value will be dropped.

// Rust string literals is a string value that hardcoded into our program.
// （硬编码到程序数据指令段，即地址为程序数据指令段的相对地址，程序加载后就固定了，而不是在堆上动态分配出来的地址，因此string literals是 immutable 的）
// string literals data type: &str
// Rust has a second string type, String, is allocated on the heap

fn main() {
    {
        let _s = "hello";
    }
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    {
        let _s = String::from("hello");
    }
    // When a variable goes out of scope, Rust automatically calls a special function `drop` for us,
    // to return back the memory

    // 5 is simple value with a known, fixed size, these two 5 values are pushed onto the stack
    // stack-only Data: Copy
    // types such as integers that have a known size at compile time are stored entirely on the stack,
    // so copies of the actual values are quick to make, in other words, there's no difference
    // between deep and shallow copying here, so we leave it out
    let x = 5;
    let y = x;
    println!("x is: {}, y is: {}", x, y);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    // there's a design choice that's implied by this: Rust will never automatically create "deep" copies

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // If a type has the Copy trait, an older variable is still usable after assignment.
    // Rust won't let us annotate a type with the Copy trait if the type, or any of its parts,
    // has implemented the Drop trait, otherwise when the value goes out of scope you'll get a compile-time error

    // any group of simple scalar values can be Copy, and nothing that requires allocation is Copy:
    // all the integer types
    // boolean type
    // all the floating point types
    // the character type, char
    // tuples, if they only contain type that are also Copy, (i32, char)

    // passing a variable to a function will move or copy, just as assignment does
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    // returning values can also transfer ownership
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    // assigning a value to another variable moves the ownership
    // When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop
    // unless the data has been moved to be owned by another variable

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
