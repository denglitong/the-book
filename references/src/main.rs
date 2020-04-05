fn main() {
    let mut s1 = String::from("hello");
    // references allow you to refer to some value without taking ownership of it, read-only refer
    // as references not own the data, the value it points to will not be dropped
    // when the reference goes out of scope
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // a mutable reference must need a mut variable
    // you can have only one mutable reference to a particular piece of data in a particular scope
    // instead like most languages let you mutate whenever you'd like, Rust can prevent data races at compile time
    change(&mut s1);

    let mut s = String::from("hello");
    {
        let s1 = &mut s;
    }
    let s2 = &mut s;

    // we also cannot have a mutable reference while we have an immutable one
    // users of an immutable reference don't expect values to suddenly change out from under them,
    // because it needs a mechanism to synchronize access to the data.

    // a reference's scope starts from where it's introduced and continues through the last time
    // that reference is used, and after that mut reference can occur because they're not the same scope
    let s1 = &mut s;
    println!("{}", s1);

    let s2 = &mut s;
    println!("{}", s2);

    // Rust compiler guarantees that reference will never be dangling references

    // slice is a different type of reference, it let you reference a contiguous sequence of elements in a collection
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    // String slice range indices must occur at valid UTF-8 character boundaries,
    // if you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error
    // the type that signifies "string slice" is written as &str

    let mut s = String::from("hello world");
    let word = first_word(&s); // 可以自动适配 String -> &str
                               // s.clear(); // mutable borrow occurs here
    println!("first word: {}", word); // immutable borrow later used here
    let word = first_word(&s[..]);
    println!("first word: {}", word);

    // string literals are slices
    // &str is a type of slice which pointing to that specific point of the binary
    // &str is an immutable reference
    let s = "Hello, world!";

    // slice is storing a reference to the beginning element and a length of collection
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice: {:?}", slice);
}

// we call having references as function parameters borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
}

// references are immutable as variables by default
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// mutable 可突变的，对应 Rust 5 个突变状态

//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// 1. At any time given, you can have either one mutable reference or any number of immutable references
// 2. References must always be valid

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // enumerate wraps the result of iter and returns each element as part of a tuple instead
    // because we get a reference to the element from .iter().enumerate(), we use & in the destructure pattern
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
