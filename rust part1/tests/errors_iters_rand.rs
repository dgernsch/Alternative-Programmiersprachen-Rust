use rust_bootcamp_part1::*;

#[test]
fn parse_and_iterators() {
    assert_eq!(parse_port("8080").unwrap(), 8080);
    assert!(parse_port("abc").is_err());

    assert_eq!(even_squares(0), vec![0]);
    assert_eq!(even_squares(6), vec![0, 4, 16, 36]);
}

#[test]
fn random_is_in_range() {
    for _ in 0..100 {
        let r = roll_dice(6);
        assert!(1 <= r && r <= 6);
    }
}

