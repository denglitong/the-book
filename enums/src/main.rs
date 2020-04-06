// Rust's enums are most similar to algebraic data types in functional languages, such as Haskell
// enums get its name because we can enumerates all the possible variants

//#[derive(Debug)]
//enum IpAddrKind {
//    V4,
//    V6,
//}
//
//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}

//fn route(ip: IpAddrKind) {}

#[derive(Debug)]
enum IpAddr {
    //    V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

// you can put any kind of data/type inside an enum variant
#[derive(Debug)]
enum Message {
    Quit,                       // no data associated
    Move { x: i32, y: i32 },    // anonymous struct
    Write(String),              // String
    ChangeColor(i32, i32, i32), // three i32 values
}
// if we use different structs, which each have their own type, we couldn't easily define a function
// to take any of these of types as we could with the enums

impl Message {
    fn call(&self) {
        println!("message call");
    }
}

// Option enum: a value should be something or it could be nothing
// the compiler can check whether you've handled all the cases you should be handling
// this functionality can prevent bugs that are extremely common in other programming languages

// Programming language design is often thought of in terms of which features you include,
// but the features you exclude are important too.
// Rust doesn't have the null feature that many other languages have.
// a null is a value that is currently invalid or absent for some reason.
// The problem isn't really with the concept but with the particular implementation.
// enum Option<T> {
//     Some(T),
//     None,
// }

// In order to have a value that can possibly be null, you must explicitly opt in
// by making the type of that value Option<T>.
// Everywhere that a value has a type that isn't an Option<T>, you can safely assume that the value isn't null.
// This was a deliberate design decision for Rust.

// In general, in order to use an Option<T> value, you want to have code that will handle each variant.

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    //    let four = IpAddrKind::V4;
    //    let _six = IpAddrKind::V6;
    //    println!("{:?}", four);

    //    let home = IpAddr {
    //        kind: IpAddrKind::V4,
    //        address: String::from("127.0.0.1"),
    //    };
    //    let loopback = IpAddr {
    //        kind: IpAddrKind::V6,
    //        address: String::from("::1"),
    //    };
    //    let home = IpAddr::V4((String::from("127.0.0.1")));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);

    let m = Message::Write(String::from("hello"));
    m.call();

    // if you want to access enums field, you need to use match pattern
    // we use &m here as we don't want to take ownership and move m into the if let pattern
    if let Message::Write(str) = &m {
        println!("Message write: {}", *str);
    }
    // as m valid here, we can use it in match pattern again, and reference not take ownership and move
    match &m {
        Message::Write(str) => println!("Message write: {}", *str),
        // _ represent other cases
        _ => {}
    }
    println!("Message: {:?}", m);

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!(
        "some_number: {:?}, absent_number: {:?}",
        some_number, absent_number
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    if let Some(i) = six {
        println!("six: {}", i);
    }
    if let Some(i) = none {
        println!("some: {}", i);
    }

    let some_u8_value = 0u8;
    match some_u8_value {
        3 => println!("three"),
        // the _ pattern will match any value
        _ => (),
    }

    // if let syntax can make less verbose way to handle that match only one pattern and ignore the rest
    // if let is syntax sugar for a match that runs code when the value matches one pattern and ignores the rest
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn value_in_cent(coin: Coin) -> u8 {
    let mut count = 0;
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from: {:?}", state);
            25
        }
    }
}

// combining match and enums is useful in many situations, you'll see this pattern a lot in Rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // match Option protects us from assuming that we have a value when we might have null
        None => None,
    }
}
