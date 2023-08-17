use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    // from trait
    let num = Number::from(30);
    println!("My number is {:?}", num);

    // into trait
    let int = 5;
    let num1: Number = int.into();
    println!("num1 is {:?}", num1);
}
