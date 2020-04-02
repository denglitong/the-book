fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let _guess: u32 = "42".parse().expect("Not a number!");

    // floating-point type default is f64 because on modern CPUs its roughly the same speed as f32
    // but is capable of more precision
    let x = 2.0;
    let y: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    // char, single quotes, 4 bytes size, represent a Unicode Scalar Value, more than ASCII
    // However, a "character" isn't really a concept in Unicode
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // tuple and array also have fixed len
    let a = [1, 2, 3, 4, 5];

    // array in Rust stored in stack, rather then the flexible vector type allocated in heap
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    another_function();
}

fn another_function() {
    println!("Another function.");
}
