pub fn fib(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => fib(n - 2) + fib(n - 2),
    }
}
