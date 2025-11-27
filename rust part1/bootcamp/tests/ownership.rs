use bootcamp::*;

#[test]
fn ownership_and_borrowing() {
    let s = String::from("hello");
    let len = take_ownership(s); // s is moved
    assert_eq!(len, 5);

    let s2 = String::from("abc");
    assert_eq!(borrow_first_char(&s2), Some('a')); // borrowed; still usable
    assert_eq!(s2, "abc");

    let mut s3 = String::from("wow");
    push_exclamation(&mut s3);
    assert_eq!(s3, "wow!");
}

