pub mod sub_mod {
    use crate::front_of_house;
    pub fn hello_world() {
        front_of_house::hosting::add_to_waitlist();
    }
}
