// By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack
// and cleans up the data from each function it encounters. But this walking back and cleanup is a lot of work.
// The alternative is to immediately abort, which ends the program without cleaning up.
// Memory that the program was using then need to be cleaned up by the operating system.
// if in your project you need to make the resulting binary as small as possible, you can switch from
// unwinding to aborting upon a panic by adding:
// [profile.release]
//  panic = 'abort'

use std::fs::File;
use std::io::{ErrorKind, Read};
use std::{fs, io, panic};

pub fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    // let f = File::open("hello.txt");
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem creating the file: {:?}", other_error),
        },
    };
    println!("{:?}", f);

    // use closure method to reduce the match expression and make code more concise
    let f = File::create("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", f);

    // let f = File::open("hello1.txt").unwrap();
    // expect is similar to unwrap and let us set the panic! error message,
    // it can help use quickly determine which code cause the panic as all panic call print stacktrace the same
    // let f = File::open("hello1.txt ").expect("Failed to open hello1.txt");

    // the ? operator can only be used in a function that returns Result( or Option or type that implement std::ops::try)
    // main function is special, one valid return type is (), another is Result<T, E>
    // fn main() -> Result<(), Box<dyn Error>> {}
    // let f = File::open("hello.txt")?;
}

// propagate errors to upstream to handle it with more context
// this pattern of propagate errors is so common in Rust that
// Rust provides the question mark operator ? to make this easier
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// the ? operator can be used in functions that return Result
fn read_username_from_file_2() -> Result<String, io::Error> {
    // the operator ? after a Result represents if Ok return the value or return Err if it fail
    // each error type implements the `from` trait to define how to convert itself to the returned error type,
    // so the ? operator takes care of the conversion automatically
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// shorter code by chain call
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// when code panics, there's no way to recover.
// you could call panic! for any error situation, whether there's a possible way to recover or not,
// but you mean that the code calling your code is unrecoverable.
// when you choose Result, you give the calling code options rather than recover it, or return Err, or panic!
// therefore, returning Result is a good choice

// panic is good for your prototyping before you're ready to decide how to handle errors, unwrap and expect are handy
// if someone calls your code and passes in values don't make sense, the best choice might be to call panic!
// Similarly, panic! is often appropriate if you're calling external code that is out of your control
// and you have no way of fixing

// however, when failure is expected, it's more appropriate to return a Result than to make a panic!
// a Result indicates that failure is an expected possibility that calling code must decide how to handle

// however, having lots fo error checks in all of your functions would be verbose and annoying.
// so Rust type system make compiler has already ensure that you have a valid value for a type
