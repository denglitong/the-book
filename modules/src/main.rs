// A package can contain multiple binary crates and optionally one library crate
// As a package grows, you can extract parts into separate crates that
// become external dependencies
// "Cargo workspace"

// Rust module systems:
// Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and user: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as struct, function, or module

// A create is a binary or library
// The crate root is a source file that the Rust compiler starts from and
// makes up the root module of your crate
// A package is one or more crates that provide a set of functionality,
// A package contains a Cargo.toml file that describes how to build those crates.

// A package must contain zero or one library crates, and no ore.
// It can contains as many binary crates as you'd like,
// but it must contain at least one crate(either library or binary)

// when we run `cargo new my-project` we create a package
// Cargo follows a convention that
// src/main.rs is the crate root of a binary crate with the same name as the package.
// Likewise, Cargo follows a convention that
// src/lib.rs is the crate root of a library crate with the same name as the package.
// Cargo passes the crate root files to rustc to build the library of binary.
// the crate root files(src/main.rs, src/lib.rs) forms a module named `crate`
// at the root of the crate's module structure, known as the module tree
// the module tree is likewise the filesystem's directory

// namely paths that allow you to name items; the use keyword that brings a path into scope;
// modules aren't useful only for organizing your code
mod front_of_house {
    // private default
    pub mod hosting {
        pub fn add_to_waitlist() {}
        // making module public don't make its content public
        // the pub keyword on a module only lets code in its ancestor modules refer to it
        fn send_to_table() {
            super::hosting::add_to_waitlist();
        }
    }

    pub mod serving {
        // Items in a parent module can't use the private items inside child modules,
        // but items in child modules can use the items in their ancestor modules
        pub fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    // In contrast, if we make en enum public, all of its variants are the public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        // public field
        pub toast: String,
        // private filed, default private
        seasonal_fruit: String,
    }

    impl Breakfast {
        // because Breakfast have private field, the struct needs to provide a public associated fn
        // that constructs an instance of Breakfast, otherwise we couldn't instance it.
        pub fn summer(toast: &str) -> Self {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

// bringing the function's parent module into scope with use is the idiomatic way
// so we have to specify the parent module while calling function makes it clear that it isn't local defined
use crate::front_of_house::hosting;
// bring a path with relative path
//use front_of_house::hosting;

//use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    // with use keyword we can bring a path into a scope and then
    // call the items in that path as if they're local items
    hosting::add_to_waitlist();
}

// otherwise, when bringing in structs, enums, and other items with use, it's idiomatic to specify the full path
// the std is also a crate that's external to our package.
// because the standard library is shipped with the Rust language, we don't need to change Cargo.toml to include std,
// but we do need to refer to it with use to bring items from there into our package's scope
use std::collections::HashMap;

//use std::fmt;
//use std::io;
//
//fn function1() -> fmt::Result {
//    Result::Ok(())
//}
//fn function2() -> io::Result<()> {
//    Result::Ok(())
//}
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    std::result::Result::Ok(())
}
fn function2() -> IoResult<()> {
    std::result::Result::Ok(())
}

// when we bring a name into scope with the use keyword, the name available in the new scope is private
// to enable the code that calls our code to refer to that name as if it had been defined in the code's scope,
// we can add `pub use path`, this technique is called re-exporting because we're bringing an item
// into scope but also making that item available for other bring into their scope
pub use crate::front_of_house::serving;

// listing packages in your package's Cargo.toml file and using use to bring items into scope

use rand::Rng;

//use std::cmp::Ordering;
//use std::io;
// using nested paths to clean up large use lists
//use std::{cmp::Ordering, io};

//use std::io;
//use std::io::Write;
//use std::io::{self, Write};

// the glob operator brings all public items defined in path
//use std::collections::*;

// restaurant/mode.rs
// pub mod restaurant in lib.rs
use modules::restaurant;

fn main() {
    // absolute path, the literal `crate` is the name of root module of the package
    // choosing whether to use a relative absolute path is a decision you'll make based on your project
    // if you would move code together, you'd rather use relative path, that's you don't need to change the path reference
    // if yo would move code separately, you'd rather use absolute path, that's the code don't moved paths keep valid
    // our preference is to specify absolute paths because it's more likely to move code definitions
    // and items calls independently of each other
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path, self:: is redundant
    self::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    eat_at_restaurant();
    serving::take_order();

    let secret_number = rand::thread_rng().gen_range(1, 101);

    restaurant::hello_world::say();
}
