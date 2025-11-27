use rust_bootcamp_part1::*;

#[test]
fn furthest_generic() {
    let pts = [
        Point { x: 1.0, y: 1.0 },
        Point { x: -5.0, y: 0.0 },
        Point { x: 2.0, y: 2.0 },
    ];
    let res = furthest_from_origin(&pts).unwrap();
    assert_eq!(*res, Point { x: -5.0, y: 0.0 });

    let tuples = [(0.0, 1.0), (3.0, 4.0), (2.0, 0.0)];
    let res2 = furthest_from_origin(&tuples).unwrap();
    assert_eq!(*res2, (3.0, 4.0));
}

