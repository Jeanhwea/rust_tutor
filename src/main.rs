#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let peter = &Person {
        name: "Peter",
        age: 23,
    };
    println!("{:?}", peter);
}
