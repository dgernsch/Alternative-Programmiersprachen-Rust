use rust_bootcamp_part1::*;

#[test]
fn basics_work() {
    assert_eq!(greet("Ada"), "Hello, Ada!");
    assert_eq!(sum(&[1, 2, 3, 4]), 10);
    assert_eq!(flip(true), false);
    assert_eq!(flip(false), true);
}

