use t21_testing::s02_integration_test as m;

#[test]
fn test_add() {
    assert_eq!(m::add(3, 2), 5);
}
