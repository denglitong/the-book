// using a semicolon after mod front_of_house tells Rust to load the contents of the module
// from another file with the same name as the module
mod front_of_house;
pub mod restaurant;

pub use crate::front_of_house::hosting;

pub fn eat_restaurant() {
    // you can keep calling path not modified as you move definition into other file
    // and brings items in this cope by using mode and use
    // this technique lets you move modules to new files as they grow in size
    hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    //    restaurant::hello_world::say();
    //    restaurant::back_of_house::serving::take_order();
}
