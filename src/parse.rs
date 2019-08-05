fn add_one(e: &mut u32) {
    *e+= 1;
}

fn add_vec(vec: &mut Vec<u32>) {
    vec.push(3);
}

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);


    add_vec(&mut vec);
    let mut i = 2;
    add_one(&mut i);
    println!("{}", i);
    println!("{:?}", vec);

    while let Some(top) = vec.pop() {
        println!("{}", top);
    }
    println!("{:?}", vec);

    let v = returnresult().unwrap();
    println!("{:?}", v);
}

fn returnresult() -> Result<&'static str, &'static str> {
    Ok("Test")
}