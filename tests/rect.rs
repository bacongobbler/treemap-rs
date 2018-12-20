extern crate treemap;

use treemap::Rect;

#[test]
fn new_rect() {
    assert_eq!(
        Rect::new(),
        Rect {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0
        }
    );
}

#[test]
fn aspect_ratio() {
    let rect = Rect::new();
    assert_eq!(rect.aspect_ratio(), 1.0);
    let rect2 = Rect::new_from_points(1.0, 1.0, 1.0, 5.0);
    assert_eq!(rect2.aspect_ratio(), 5.0);
}
