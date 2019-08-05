use std::collections::HashMap;

fn main() {
    let mut has: HashMap<String, HashMap<i32, String>> = HashMap::new();

    has.insert("Test".to_string(), HashMap::new());
    for i in 1..10 {
        match has.get_mut("Test") {
            Some(value) => {
                value.insert(i, "Test".to_string());
            },
            None => {
                has.insert("Test".to_string(), HashMap::new());
            }
        };
    }

    println!("{:?}", has);
}