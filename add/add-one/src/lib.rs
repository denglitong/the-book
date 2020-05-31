use rand;

pub fn plus_one(x: i32) -> i32 {
    x + 1
}

pub fn test_rand_v5() {
    let x: u8 = rand::random();
    println!("random: {}", x);
}

#[cfg(test)]
mod tests {
    use crate::plus_one;

    #[test]
    fn it_works() {
        assert_eq!(3, plus_one(2));
    }
}