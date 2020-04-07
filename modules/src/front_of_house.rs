// Rust lets you split a package into multiple crates and a crate into modules
// so you can refer to items defined in one module from another module.
pub mod hosting {
    pub fn add_to_waitlist() {}
}

use crate::restaurant::sub::{self, sub_mod};

pub fn test() {
    sub::sub_mod::hello_world();
    sub_mod::hello_world();
}
