use std::fmt::Display;

// lifetimes is another kind of generic.
// rather than ensuring that a type has the behavior we want, lifetimes ensure that
// references are valid as long as we need them to be.
// the main aim of lifetimes is to prevent dangling references
pub fn main() {
    // you can declare not-initialized var but must initialized before you use it
    let r: i32;
    {
        let x = 5;
        let r = &x;
    }
    // rust borrow checker x doesn't live long enough
    // println!("r: {}", r);
    let x = 5;
    let r = &x;
    println!("r: {}", r);

    let str1 = String::from("abcd");
    let str2 = "xyz";

    let result = longest(str1.as_str(), str2);
    println!("The longest string is {}", result);

    let str1 = String::from("long string is long");
    {
        let str2 = String::from("xyz");
        let result = longest(str1.as_str(), str2.as_str());
        println!("The longest string is {}", result);
    }

    // let result;
    // {
    //     let str2 = String::from("xyz");
    //     result = longest(str1.as_str(), str2.as_str());
    // }
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);
    println!("{:?}", first_sentence); // &str just borrow above, so it's valid here

    let s: &'static str = "I have a static lifetime.";

    let result = longest_with_an_announcement(str1.as_str(), str2, first_sentence);
    println!("The longest string is: {}", result);
}

// the lifetimes of params and return live long the same
// rust compiler needs our help to make it clear of the lifetimes generics
// to let borrow checker works fine.
// the syntax <'a> is generics syntax, just like generic types <T>

// the function signature means the return lifetime live as long as the shortest of params
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// if return type lifetime is the same as the first params, the lifetime annotation is unnecessary.
// the lifetime of returning type from a function needs to match the lifetime of one of params.
// (because the input params bring the context lifetime of params into function then return types can specific and borrow checker can work)
// otherwise, the retuning types may refer to reference inner function which will be a dangling ref
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// lifetime syntax is about connecting the lifetimes of various params and return values of functions.
// then Rust has enough information to allow memory-safe operations and disallow operations that
// would create dangling pointers

// Rust lifetime must apply to reference type, because we only have borrows in reference, not value types
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
    // content: String, // won't need lifetime annotation, ownership moves
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// lifetime elision (缺省)
// as lifetime is to connecting the returning types scope with input params scope,
// so if it has only one params, the lifetime of returning types are the same as the input params
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// input lifetimes from input params, output lifetimes of returning types
// lifetime elision rules:
// 1. each params that is a reference gets its own lifetime parameter.
// 2. if there is exactly one input lifetime parameter(that's only one reference input params),
//      then the returning types lifetimes are the same of the input ref params
// 3. if there are multiple input lifetime params and one of them is &self or &mut self,
//      then the returning types lifetimes are the same of the &self/&mut self
// Otherwise, after go through the three rules above, the compiler cannot determine the lifetime itself,
// then it's time for us to specify the lifetime annotation.

// 'static lifetime means the lifetimes of reference can live for the entire duration of the program.
// all string slice have the 'static lifetime

// lifetimes 'a must be declared prior to generic type T
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement!: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
