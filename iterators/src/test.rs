#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        match self {
            Quit => println!("Q"),
            Message::Write(s) => println!("this is a write{:?}", s),
            _ => (),
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
    println!("Hello, world!");
}
