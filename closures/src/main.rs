use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);

    // we want to define code in one place in our program,
    // but only execute that code where we actually need the result.
    // this is a use case for closure
    // closures are usually short and relevant only within a narrow context
    // rather than in any arbitrary scenario
    // let expensive_closure = |num: u32| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!", expensive_result.value(intensity));
    }

    let x = 4;
    // when a closure captures a value from its environment, it uses memory to store the values for use
    // in the closure body.
    // closures capture values from their environment in three ways: (same as functions take params)
    // 1. taking ownership: FnOnce
    //  consumes closure's environment, which must take ownership and move into the closure.
    //  the Once part of the name represents the closure can't take ownership of the same variables more than once
    // 2. borrowing mutably: FnMut
    //  can change the closure environment because it mutably borrows values
    // 3. borrowing immutably: Fn
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    let another = String::from("str");
    // move keyword force closure to take ownership of the values it uses in the environment
    let equal_to_x = move |z| {
        println!("another: {:?}", another);
        z == x
    };
    // println!("can't use x here: {:?}", x);
    // println!("can't use another here: {:?}", another);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// if what we want to do doesn't require capturing a value from the environment,
// we can use a function rather than a closure
struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}