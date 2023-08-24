#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let jack = &Person {
        name: "Jack".to_string(),
        age: 23,
    };
    println!("Hello, Person = {:?}", jack);
}
