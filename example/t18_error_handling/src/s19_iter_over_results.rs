fn main() {
    let strings = vec!["tofu", "93", "18"];
    // v1: 使用 map 遍历可能会报错
    // let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();

    // v2: 使用 filter_map 过滤掉错误的数据
    let numbers_2: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("numbers_2: {:?}", numbers_2);

    // v3: 在遍历的过程中收集错误
    let strings_3 = vec!["42", "tofu", "93", "18"];
    let mut errors = vec![];
    let numbers_3: Vec<_> = strings_3
        .clone()
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("numbers_3: {:?}, errors:{:?}", numbers_3, errors);

    // v4: 使用 Result<Vec<T>, E> 作为返回值, 如果 collect() 第一次出现错误就会终止
    let strings_4 = vec!["42", "banana", "tofu", "93", "18"];
    let numbers_4: Result<Vec<_>, _> = strings_4.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("numbers_4: {:?}", numbers_4);

    // v5: 使用 partition() 来将错误分类
    let strings_5 = vec!["tofu", "93", "18", "bad"];
    let (numbers_5, err_part): (Vec<_>, Vec<_>) = strings_5
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("numbers_5: {:?}, err_part: {:?}", numbers_5, err_part);
}
