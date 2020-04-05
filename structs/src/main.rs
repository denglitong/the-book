// structs vs tuples
// struct name piece of data so it's clear what the values mean.
struct User {
    // we used the owned String type rather than the &str string slice type.
    // This is a deliberate choice because we want instances of this struct to own all of its data
    // and for that data to be valid for as long as the entire struct is valid.
    // otherwise for structs to store references to data owned by something else
    // requires the use of lifetimes. Lifetimes ensure that the data referenced by a struct
    // is valid for as long as the struct is.
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// you can also define structs that look like similar to tuples, called tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// you can also define structs that don't have any fields, which are called unit-like structs
// because they behave similarly to (), the unit type.
// Unit-like structs can be useful in situations in which you need to implement a trait on some type
// but you don't have any data that you want to store in the type itself.

// you can use derive traits for type
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods are different from functions in that they're defied within the context of a struct/type,
// and their first parameter is always self, which represents the instance of the struct/type the method is being called on
// impl block can have multiples
impl Rectangle {
    // methods can take ownership of self, there we borrow self immutably
    // take self ownership is usually used when the method transforms self in to something else
    // and you want to prevent the caller from using the original instance after the transformation
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // in C/C++, if object is a pointer, object->method() is similar to (*object).method()
    // in Rust, it has a feature called automatic referencing and dereferencing.
    // when you call a method with object.method(), Rust automatically adds in &, &mut, or *
    // so object matches the signature of the method.
    // Rust can figure out definitively whether the method is reading(&self), mutating(&mut self) or consuming(self)

    // we're allowed to define functions that don't take self as a parameter,
    // these are called associated functions because they're associated with the struct/type
    // they're still functions, not methods, because they don't have an instance context of struct/type to work with
    // That's, method work with struct/type instance, associated functions just work with struct/type itself
    // associated functions are often used for constructors
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, rhs: &Self) -> bool {
        self.width > rhs.width && self.height > rhs.height
    }

    // error: duplicate definitions, cannot override
    //    fn square(size: u32) -> Rectangle {
    //        Rectangle {
    //            width: size,
    //            height: size,
    //        }
    //    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // noth that the entire instance must be mutable, Rust doesn't allow us to mark only certain
    // fields as mutable
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // the syntax .. specifies that the remaining fields not explicitly set
        // should be the same as the fields in the given instance
        ..user1
    };

    // tuple structs 可以理解为 tuple 的具名type
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );

    // the curly brackets {} tell println! to use formatting known as Display
    // println!("rect1 is {}", rect1);
    // Putting the specifier :? inside {} tells println! we want to use an output format called Debug
    println!("rect1 is {:?}", rect1);
    // :#? 会格式化输出
    println!("rect1 is {:#?}", rect1);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("The square is {:?}", Rectangle::square(3));
    println!("The area of square is {}", Rectangle::square(3).area());
}

fn _build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Rust 不支持同名函数重载
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
