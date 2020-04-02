fn main() {
    let number = 3;

    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    // Rust has three kinds of loops: loop, while and for
    // loop {
    //     println!("again!");
    // }
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // pass value you want returned after the break expression
            break counter * 2
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // for iter, its safety and conciseness make it the most commonly used loop construct in Rust
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // reverse iter
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    println!("100 fathrenheith is celsius: {}", fahrenheitToCelsius(100.0));
    println!("100 celsius is fathrenheith: {}", celsiusToFahrenheit(100.0));

    for n in 1..11 {
        print!("{} ", fibonacci(n));
    }
    println!();

    the_twelve_days_of_christmas();
}

fn fahrenheitToCelsius(fahr: f64) -> f64 {
    (fahr - 32.0) / 1.8
}

fn celsiusToFahrenheit(cels: f64) -> f64 {
    cels * 1.8 + 32.0
}

fn fibonacci(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    }

    let (mut a, mut b) = (1, 1);
    let mut counter = 2;
    while counter < n {
        let c = b;
        b = a + b;
        a = c;
        counter += 1;
    }

    b
}

fn the_twelve_days_of_christmas() {
    let days_sequence = ["fist", "second", "third", "fourth", "fifth", "sixth",
                                "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let mut index = 0;
    while index < days_sequence.len() {
        println!("On the {} day of Christmas, my true love sent to me: {}", days_sequence[index],
                 the_twelve_days_of_christmas_send(index + 1));
        index += 1;
    }
}

fn the_twelve_days_of_christmas_send(n: usize) -> String {
    let sends_sequence = [
        "A partridge in a pear tree", "Two turtle doves", "Three French hens",
        "Four calling birds", "Five golden rings", "Six geese a-laying",
        "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing",
        "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"
    ];

    let mut sends = String::new();
    for index in (1..n).rev() {
        sends.push_str(sends_sequence[index]);
        sends.push_str(", ");
    }
    sends.push_str(sends_sequence[0]);
    sends.push_str(".");

    sends
}