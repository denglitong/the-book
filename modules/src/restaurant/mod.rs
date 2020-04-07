pub mod hello_world {
    pub fn say() {
        println!("This is restaurant hello_world mod and say function");
    }
}

// re-exporting
// take back_of_house module in same directory with name back_of_house.rs
// or back_of_house/mode.rs
pub mod back_of_house;

pub mod sub;
