use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    marks.insert("Rust Programming", 96);
    marks.insert("Golang Programming", 96);
    marks.insert("Web Development", 94);


    // Get value
    match &marks.get("Web Development") {
        Some(mark) => println!("{}", mark),
        None => println!("None")
    }

    for (_, mark) in &marks {
        println!("{}", mark);
    }

    marks.remove("Web Development");


    println!("{}", marks.contains_key("Golang Programming"));
}