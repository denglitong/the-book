use crate::rc::List::Cons;
use std::rc::Rc;

// we use the `Rc<T>` type when we want to allocate some data on the heap
// for multiple parts of our program to read.
// Note that Rc<T> is only for use in single-threaded scenarios.
// via immutable references, Rc<T> allows you to share data between multiple parts
// of your program for reading only.
pub fn main() {
    let a = List::Cons(5, Box::new(Cons(10, Box::new(List::Nil))));
    let b = List::Cons(3, Box::new(a));
    // use of moved data
    // let c = List::Cons(4, Box::new(a));

    let ten = ListRef::Cons(10, Box::new(&ListRef::Nil));
    let ra = ListRef::Cons(5, Box::new(&ten));
    let rb = ListRef::Cons(3, Box::new(&ra));
    let rc = ListRef::Cons(4, Box::new(&ra));
    println!("{:?}", rb);
    println!("{:?}", rc);

    let five = ListRc::Cons(5, Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil))));
    // Rc::new use up and move
    let rca = Rc::new(five);
    println!("count after creating a = {}", Rc::strong_count(&rca));
    let rcb = ListRc::Cons(3, Rc::clone(&rca));
    println!("count after creating b = {}", Rc::strong_count(&rca));
    {
        let rcc = ListRc::Cons(4, Rc::clone(&rca));
        println!("count after creating c = {}", Rc::strong_count(&rca));
    }
    println!(
        "count after c goes out of scope = {}",
        Rc::strong_count(&rca)
    );

    // println!("{:?}", rcb);
    // println!("{:?}", rcc);

    // let rcd = ListRc::Cons(6, Rc::new(five));
}

#[derive(Debug)]
enum List {
    // own value take ownership, will use up params and move
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum ListRef<'a> {
    // reference don't take ownership, introduce lifetime annotation
    Cons(i32, Box<&'a ListRef<'a>>),
    Nil,
}

#[derive(Debug)]
enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}
