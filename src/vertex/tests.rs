use image::{ImageBuffer, Rgb, RgbImage};
use crate::vertex::{Vertex, point::Point, self, draw_verticies};


#[test]
fn test_is_inside() {
    let p1 = Point::new(11, 10);
    let p2 = Point::new(10, 50);
    let p3 = Point::new(50, 10);

    let test_point = Point::new(20, 20);

    let v1 = Vertex::from_points(&p1, &p2, &p3);
    let v2 = Vertex::from_points(&p1, &p3, &p2);
    let v3 = Vertex::from_points(&p2, &p1, &p3);


    assert!(v1.d1.0.is_over(&test_point) == v1.d1.1);
    assert!(v1.d2.0.is_over(&test_point) == v1.d2.1);
    assert!(v1.d3.0.is_over(&test_point) == v1.d3.1);
    assert!(v1.is_inside(&test_point));
    assert!(v2.is_inside(&test_point));
    assert!(v2.d1.0.is_over(&test_point) == v2.d1.1);
    assert!(v2.d2.0.is_over(&test_point) == v2.d2.1);
    assert!(v2.d3.0.is_over(&test_point) == v2.d3.1);
    assert!(v3.is_inside(&test_point));
} 