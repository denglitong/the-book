use crate::traits::Summary;

struct A {}

// you must bring Summary to the scope to implement and use methods,
// bring upto the trait is not enough, bring to the trait itself is need.
// One restriction to note with trait implementations is that
// we can implement a trait on a type only if either the trait or the type is local.
// this restriction is part of a property of programs called coherence(连贯性),
// or more specifically the orphan rule(孤儿规则)
impl Summary for A {
    fn summarize(&self) -> String {
        format!("This is A struct")
    }

    fn summarize_author(&self) -> String {
        String::from("someone")
    }
}

pub fn main() {
    let a = A {};
    println!("{}", a.summarize());
}
