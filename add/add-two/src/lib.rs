pub fn plus_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod test {
    use crate::plus_two;

    #[test]
    fn it_works() {
        assert_eq!(4, plus_two(2));
    }
}