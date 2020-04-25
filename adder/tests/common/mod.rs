// naming file in dir/mod.rs tells Rust
// not to treat the common module as an integration test file
// Files in subdirectories of the tests directory don't get compiled
// as separate crates or have sections in the test output

mod local;

pub fn setup() {
    local::setup();
}
