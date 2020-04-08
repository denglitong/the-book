pub fn main() {
    // an important point is that vectors are implemented using generics
    let v: Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);

    let mut v = Vec::new();
    v.push(1);
    v.push(2);

    {
        let v = vec![1, 2, 3, 4];
    } // dropping a vector drops its elements

    let v = vec![1, 2, 3, 4, 5];
    // we use & here as we just want to refer and not need to care about take owner
    // if index is outbound here program will crash
    // immutable borrow
    let third = &v[2];
    println!("The third element is {}", third);

    // if index is outbound here, it will return None rather than crash, you'll always use .get()
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // use enum to store multiple types in vector
    // Rust needs to know what types will be in the vector at compile time
    // so it knows exactly how much memory on the heap will be needed to store each element
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);
    // pop removes and returns the last element
    let p = v.pop();
    println!("{:?}", p.unwrap());
}
