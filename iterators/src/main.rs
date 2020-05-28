fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // we don't need to make `v1_iter` mut when we used a `for` loop
    // because the loop took ownership of `v1_iter` and made it mutable behind the scenes
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // the `iter` method produces an iterator over immutable references
    // if we want to iterate over mutable references, we can call `iter_mut`
    let mut v1 = vec![1, 2, 3];
    let v1_iter_mut = v1.iter_mut();
    for val in v1_iter_mut {
        *val = *val + 1;
    }
    println!("after iter mut: {:?}", v1);

    // if we want to take ownership of `v1` and returns owned values, we can call `into_iter`
    let v1 = vec![1, 2, 3];
    let v1_into_iter = v1.into_iter();
    let consume = move |z| {
        println!("consume: {}", z);
        z
    };
    for val in v1_into_iter {
        println!("Got owned: {}", val);
        consume(val);
    }
    // value borrowed after move
    // println!("can't access v1: {:?}", v1);

    let v1 = vec![1, 2, 3];
    // `map`, `collect` also consuming adaptors
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    for c in Counter::new() {
        println!("got: {}", c);
    }
}

pub trait MyIterator {
    // associated type
    type Item;

    // consuming adaptors, uses up the iterator
    fn next(&mut self) -> Option<Self::Item>;

    // consuming adaptors, takes ownership of the iterator
    fn sum(self) -> Self::Item;
}

#[cfg(test)]
mod test {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        // call `next` need mut self
        let mut v1_iter = v1.iter();
        // this code consumes the iterator
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        // nothing output, v1_iter has state and already reach the end
        for v in v1_iter {
            println!("test Got: {}", v);
        }
        println!("test end");
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        // `sum` use up v1_iter
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // 这里要返回值需要取得所有权，所以使用了 into_iter()，是 consuming adaptors
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        // `zip` only pairs and if encounter None that item will not be included
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(18, sum);
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            return Some(self.count);
        }
        None
    }
}
