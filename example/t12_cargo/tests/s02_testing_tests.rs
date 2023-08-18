use t12_cargo::s02_testing as t;

#[test]
fn test_fib_calls() {
    assert_eq!(1, t::fib(0));
}

#[test]
fn test_fib_calls_2() {
    assert_eq!(2, t::fib(2));
}
