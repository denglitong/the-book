//! Box<T>, Deref, Drop

// Rust needs to know the specific size at compile time.
// One type whose size can't be known at compile time is a recursive type,
// where a value can have as part of itself another value of the same type.
// Rust can't known the recursive type size, but boxes have a known size,
// so by inserting a box in a recursive type definition, you can have recursive types.

// Boxes provide only the indirection and heap allocation,
// they don't have any other special capabilities,
// boxes are smart pointers because they implement `Drop` trait and deallocate boxes and data
// when box get out of scope.

use std::ops::Deref;

pub fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let one = List::Cons(1, Box::new(List::Nil));
    let _two = List::Cons(2, Box::new(one));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // compare a number with a reference to a number isn't allowed because they're different types
    // assert_eq!(5, y);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    // Box<T> implement `Deref` trait so it has dereference behaviour
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // *(y.deref())
    assert_eq!(5, *y);

    hello("Rust");
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let s = &(*m)[..];
    hello(s);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// treating a type like a reference by implementing the `Deref` trait.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Deref coercion interacts with mutability
// &T -> &U when T: Deref<Target=U>
// &mut T -> &mut U when T: DerefMut<Target=U>
// &mut T -> &U when T: Deref<Target=U>

// Deref coercion, 强制解引用
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // usually you would specify they cleanup code that your type needs
        // rather than a print message
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
