use std::collections::HashMap;

// HashMap<K, V>
pub fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores.get("Blue"));
    // println!("{:?}", scores.get(String::from("Yellow"))); // expected &str, found String

    // we can construct HashMap by using the collect method on a vector of tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // we use HashMap<_,_> because Rust can infer the types based on the types of the data in te vec
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    // for types that implement the Copy trait, like i32, the values are copied into the hash map.
    // for owned data like String, the values will be moved and the hash map will be the owner of those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{} {}", field_name, field_value); // error: value borrow after moved

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwriting a value
    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores);
    // only insert a value if the key has no value
    // scores is mut borrow, and its scope will end up when the statement is finished,
    // so it can occur again in next state also as mut borrow
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // the or_insert method actually returns a mut reference to the value(&mut V)
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
