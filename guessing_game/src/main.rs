use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // random number generator
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // two lines for two method calls,
        // stdin is a type that represents a handle to the standard input for your terminal
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // if Result is Err, expect will cause the program to crash and display the msg you passed
        // if Result is OK, expect will take the value the Ok is holding and return just that value

        // "5\n".trim() = 5, trim eliminates \n
        // parse method on strings parses a string into some kind of number
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will between 1 and 100.");
            continue;
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
