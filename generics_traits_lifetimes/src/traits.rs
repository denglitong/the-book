// A trait tells the Rust compiler about functionality a particular type has
// and can share with other types. We can use traits to define shared behavior
// in an abstract way. We can use trait bounds to specify that
// a generic can be any type that has certain behavior.

use std::fmt::{Debug, Display};
use std::os::macos::raw::time_t;

pub trait Summary {
    // fn summarize(&self) -> String;
    // default implementation
    fn summarize(&self) -> String {
        // default implementation can call other method
        format!("Read more from {}...", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
    // Note that isn't possible to call the default implementation
    // from an overriding implementation of the same method.
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as yo probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 enw tweet: {}", tweet.summarize());

    let s = returns_summarizable3(true);
    println!("{}", s.summarize());
    let s = returns_summarizable3(false);
    println!("{}", s.summarize());

    let s = "123";
    println!("{}", s.to_string());
}

// traits as parameters, use the impl Trait syntax, it means the fn accept any types
// which implement the trait.
// The impl Trait syntax works for straightforward cases but is actually syntax
// for a longer form, which is called a trait bound.
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// the trait bound syntax is verbose so we don't use it
pub fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}

// item1 and item2 is some type of implementation of Summary, they're not must the same type
pub fn notify3(item1: impl Summary, item2: impl Summary) {}

// item1 and item2 must be the same type that implements Summary
pub fn notify4<T: Summary>(item1: T, item2: T) {}

// specifying multiple trait bounds with the + syntax
// + syntax means AND logic
pub fn notify5(item: impl Summary + Display) {}
pub fn notify6<T: Summary + Display>(item: T) {}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}
// clear trait bounds with where clauses
fn some_function2<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

// traits as returning types
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// however, you can only use impl Trait when you're returning a single type implementation,
// if you want to return multiple types implementation the compiler wil catch a error:
// that's not allow due to restrictions around how the impl Trait syntax is implemented
// error: `if` and `else` have incompatible types
//fn returns_summarizable2(switch: bool) -> impl Summary {
//    if switch {
//        NewsArticle {
//            headline: String::from("Penguins win the Stanley Cup Championship!"),
//            location: String::from("Pittsburgh, PA, USA"),
//            author: String::from("Iceburgh"),
//            content: String::from(
//                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
//            ),
//        }
//    } else {
//        Tweet {
//            username: String::from("horse_ebooks"),
//            content: String::from("of course, as you probably already know, people"),
//            reply: false,
//            retweet: false,
//        }
//    }
//}

// use Box<syn Summary> means return a dynamic type which implement Summary in a Box
// a Box type is a pointer type for heap allocation
// in this way we can return multiple trait objects
fn returns_summarizable3(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        })
    } else {
        Box::new(Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        })
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

// methods implement for type without condition bounds
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x, y }
    }
}

// using trait bounds to conditionally implement methods
impl<T: PartialOrd + Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}

// we can also conditionally implement a trait for any type T
// the std library implements the ToString trait on any type
//impl<T: Display> ToString for T {
//    fn to_string(&self) -> String {
//        // format!("this is my to_string: {}", self)
//    }
//}
