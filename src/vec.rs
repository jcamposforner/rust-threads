use std::collections::HashMap;

fn main() {
    let mut has: HashMap<String, Vec<i32>> = HashMap::new();

    has.insert("Test".to_string(), Vec::new());
    for i in 1..10 {
        match has.get_mut("Test") {
            Some(value) => {
                value.push(i);
            },
            None => {
                has.insert("Test".to_string(), Vec::new());
            }
        };
    }

    println!("{:?}", has);
}