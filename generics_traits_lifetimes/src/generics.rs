#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// we have to declare T just after impl so we can use it to specify that
// we're implementing methods on the type Point<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// we can also implement method on concrete type rather than generics
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// generic type parameters in a struct definition
// aren't always the same as those you use in that struct's method signatures
impl<T, U> Point2<T, U> {
    // Rust implements generics in such a way that your code doesn't run any slower using generic types
    // this is accomplishes by performing monomorphization of the code that is using generics at compile time.
    // it generic code into specific code by filling in the concrete types that are used when compiled.

    // move ownership
    fn mixup<V, W>(self, rhs: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: rhs.y,
        }
    }
}

pub fn main() {
    //    let number_list = vec![34, 50, 25, 100, 65];
    //    let result = largest_i32(&number_list);
    //    println!("The largest number is {}", result);
    //
    //    let char_list = vec!['y', 'm', 'a', 'q'];
    //    let result = largest_char(&char_list);
    //    println!("The largest char is {}", result);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let result = largest_ref(&number_list);
    println!("The largest number is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer: {:?}, float: {:?}", integer, float);

    let integer = Point2 { x: 5, y: 10 };
    let float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!(
        "integer: {:?}, float: {:?}, integer_and_float: {:?}",
        integer, float, integer_and_float
    );

    let p = Point { x: 5, y: 5 };
    println!("p.x = {}", p.x());
    let f: Point<f32> = Point { x: 1.0, y: 4.0 };
    println!("p distance from origin: {}", f.distance_from_origin());
    // p don't have distance_from_origin() method,
    // and f need to specify type Point<f32> as to call distance_from_origin()

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y: {}", p3.x, p3.y);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// if you use Clone instead of Copy, it's potentially making more heap allocations in case of types that
// own heap data like String, and heap allocations can be slow if we're working with large amounts of data.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        // the greater than > operator is defined as a default method on the std library trait std::cmp::PartialOrd
        if item > largest {
            largest = item;
        }
    }
    largest
}

// return ref we don't need Copy bounds
fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
