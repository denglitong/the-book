fn main() {
    another_function(5, 6);

    // Rust is an expression-based language, this is an important distinction to understand
    // statement vs expression
    // statement are instructions the perform some action and do not return a value
    // Expression evaluate to a resulting value, expression can be part of statement
    let y = 6;
    // let x = (let y = 6); // `let y = 6` statement does not return a value, nothing to bind to x

    let x = 5;
    let y = {
        let x = 3;
        // expressions do not include ending semicolons, if added it turned into a statement
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// requiring type annotations in function definitions means
// the compiler almost never needs you to use them elsewhere in the code to figure out what you mean
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}