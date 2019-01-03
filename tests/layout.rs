extern crate treemap;

use treemap::{MapItem, Mappable, Rect, TreemapLayout};

#[test]
fn layout_items() {
    let bounds = Rect::new_from_points(0.0, 0.0, 6.0, 4.0);
    let mut items: Vec<Box<Mappable>> = vec![
        Box::new(MapItem::new_with_size(6.0)),
        Box::new(MapItem::new_with_size(6.0)),
        Box::new(MapItem::new_with_size(4.0)),
        Box::new(MapItem::new_with_size(3.0)),
        Box::new(MapItem::new_with_size(2.0)),
        Box::new(MapItem::new_with_size(2.0)),
        Box::new(MapItem::new_with_size(1.0)),
    ];

    let expected_output = vec![
        Rect::new_from_points(0.0, 0.0, 3.1304347826086953, 2.0),
        Rect::new_from_points(0.0, 2.0, 3.1304347826086953, 2.0),
        Rect::new_from_points(
            3.1304347826086953,
            0.0,
            2.8695652173913047,
            1.4545454545454546,
        ),
        Rect::new_from_points(
            3.1304347826086953,
            1.4545454545454546,
            2.459627329192547,
            1.5272727272727271,
        ),
        Rect::new_from_points(
            3.1304347826086953,
            2.9818181818181815,
            2.459627329192547,
            1.0181818181818183,
        ),
        Rect::new_from_points(
            5.590062111801242,
            1.4545454545454546,
            0.4099378881987579,
            1.6969696969696968,
        ),
        Rect::new_from_points(
            5.590062111801242,
            3.1515151515151514,
            0.4099378881987579,
            0.8484848484848484,
        ),
    ];

    let mut layout = TreemapLayout::new();
    layout.layout_items(&mut items, bounds);

    for i in 0..items.len() {
        let item_bounds = items[i].get_bounds();
        assert_eq!(expected_output[i].x, item_bounds.x);
        assert_eq!(expected_output[i].y, item_bounds.y);
        assert_eq!(expected_output[i].w, item_bounds.w);
        assert_eq!(expected_output[i].h, item_bounds.h);
    }
}

#[test]
fn sort_descending() {
    let input = [
        24.0, 2.0, 45.0, 20.0, 56.0, 75.0, 2.0, 56.0, 99.0, 53.0, 12.0,
    ];
    let output = [
        99.0, 75.0, 56.0, 56.0, 53.0, 45.0, 24.0, 20.0, 12.0, 2.0, 2.0,
    ];
    let mut items: Vec<Box<Mappable>> = Default::default();
    for i in input.iter() {
        items.push(Box::new(MapItem::new_with_size(*i)));
    }
    treemap::sort_descending(&mut items);
    for i in 0..items.len() {
        assert_eq!(output[i], items[i].get_size());
    }
}
