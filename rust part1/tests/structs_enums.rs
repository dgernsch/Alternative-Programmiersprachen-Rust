use rust_bootcamp_part1::*;

#[test]
fn structs_enums_methods() {
    let a = Point { x: 0.0, y: 0.0 };
    let b = Point { x: 3.0, y: 4.0 };
    assert!((a.distance_to(&b) - 5.0).abs() < 1e-9);

    let c = Shape::Circle { center: Point::origin(), radius: 2.0 };
    let r = Shape::Rect { top_left: Point::origin(), w: 3.0, h: 4.0 };

    let pi = std::f64::consts::PI;
    assert!((c.area() - (pi * 4.0)).abs() < 1e-9);
    assert!((r.area() - 12.0).abs() < 1e-9);
}

