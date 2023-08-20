use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
}

// 和上面函数不同的是, 将错误包裹到最外层
fn double_first_2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty));
    println!("The first doubled is {:?}", double_first(strings));

    let numbers_2 = vec!["42", "93", "18"];
    let empty_2 = vec![];
    let strings_2 = vec!["tofu", "93", "18"];
    println!("The first doubled is {:?}", double_first_2(numbers_2));
    println!("The first doubled is {:?}", double_first_2(empty_2));
    println!("The first doubled is {:?}", double_first_2(strings_2));
}
