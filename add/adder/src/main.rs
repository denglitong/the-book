use add_one;
use add_two;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::plus_one(num),
    );

    add_one::test_rand_v5();

    // workspace crates can use different dependency version on same cratecar
    test_rand_v7();

    println!(
        "Hello, world! {} plus two is {}!",
        num,
        add_two::plus_two(num),
    );
}

pub fn test_rand_v7() {
    let x: u16 = rand::random();
    println!("random: {}", x);
}