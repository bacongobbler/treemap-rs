extern crate treemap;

use treemap::{MapItem, Mappable, Rect, TreemapLayout};

#[test]
fn treemap() {
    let mut layout = TreemapLayout::new();
    let bounds = Rect::new_from_points(0.0, 0.0, 6.0, 4.0);
    let mut items: Vec<Box<Mappable>> = vec![
        Box::new(MapItem::new_from_size_and_order(6.0, 0)),
        Box::new(MapItem::new_from_size_and_order(6.0, 0)),
        Box::new(MapItem::new_from_size_and_order(4.0, 0)),
        Box::new(MapItem::new_from_size_and_order(3.0, 0)),
        Box::new(MapItem::new_from_size_and_order(2.0, 0)),
        Box::new(MapItem::new_from_size_and_order(2.0, 0)),
        Box::new(MapItem::new_from_size_and_order(1.0, 0)),
    ];

    layout.layout_items_at(&mut items, 0, 6, bounds);

    for i in 0..7 {
        println!("Item {} x={}", i, items[i].get_bounds().x);
        println!("Item {} y={}", i, items[i].get_bounds().y);
        println!("Item {} w={}", i, items[i].get_bounds().w);
        println!("Item {} h={}", i, items[i].get_bounds().h);
        println!("------");
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
        items.push(Box::new(MapItem::new_from_size_and_order(*i, 0)));
    }
    treemap::sort_descending(&mut items);
    for i in 0..items.len() {
        assert_eq!(output[i], items[i].get_size());
    }
}

#[test]
fn highest_aspect() {
    let layout = TreemapLayout::new();
    let bounds = Rect::new_from_points(0.0, 0.0, 6.0, 4.0);
    let mut items: Vec<Box<Mappable>> = vec![
        Box::new(MapItem::new_from_size_and_order(6.0, 0)),
        Box::new(MapItem::new_from_size_and_order(6.0, 0)),
        Box::new(MapItem::new_from_size_and_order(4.0, 0)),
    ];

    assert_eq!(4.0 / 1.5, layout.highest_aspect(&mut items, 0, 1, &bounds));
    assert_eq!(3.0 / 2.0, layout.highest_aspect(&mut items, 0, 2, &bounds));
    assert_eq!(4.0 / 1.0, layout.highest_aspect(&mut items, 0, 3, &bounds));
}

#[test]
fn layout_row_horizontal() {
    let layout = TreemapLayout::new();
    let bounds = Rect::new_from_points(0.0, 0.0, 6.0, 4.0);
    let mut items: Vec<Box<Mappable>> = vec![
        Box::new(MapItem::new_from_size_and_order(6.0, 0)),
        Box::new(MapItem::new_from_size_and_order(6.0, 0)),
        Box::new(MapItem::new_from_size_and_order(4.0, 0)),
    ];

    let output1 = vec![Rect::new_from_points(0.0, 0.0, 1.5, 4.0)];
    let output2 = vec![
        Rect::new_from_points(0.0, 0.0, 3.0, 2.0),
        Rect::new_from_points(0.0, 2.0, 3.0, 2.0),
    ];
    let output3 = vec![
        Rect::new_from_points(0.0, 0.0, 4.0, 1.5),
        Rect::new_from_points(0.0, 1.5, 4.0, 1.5),
        Rect::new_from_points(0.0, 3.0, 4.0, 1.0),
    ];

    let rect1 = layout.layout_row(&mut items, 0, 1, &bounds);

    assert_eq!(1.5, rect1.x);
    assert_eq!(0.0, rect1.y);
    assert_eq!(4.5, rect1.w);
    assert_eq!(4.0, rect1.h);

    assert_eq!(output1[0].x, items[0].get_bounds().x);
    assert_eq!(output1[0].y, items[0].get_bounds().y);
    assert_eq!(output1[0].w, items[0].get_bounds().w);
    assert_eq!(output1[0].h, items[0].get_bounds().h);

    let rect2 = layout.layout_row(&mut items, 0, 2, &bounds);

    assert_eq!(3.0, rect2.x);
    assert_eq!(0.0, rect2.y);
    assert_eq!(3.0, rect2.w);
    assert_eq!(4.0, rect2.h);

    for i in 0..2 {
        assert_eq!(output2[i].x, items[i].get_bounds().x);
        assert_eq!(output2[i].y, items[i].get_bounds().y);
        assert_eq!(output2[i].w, items[i].get_bounds().w);
        assert_eq!(output2[i].h, items[i].get_bounds().h);
    }

    let rect3 = layout.layout_row(&mut items, 0, 3, &bounds);

    assert_eq!(4.0, rect3.x);
    assert_eq!(0.0, rect3.y);
    assert_eq!(2.0, rect3.w);
    assert_eq!(4.0, rect3.h);

    for i in 0..2 {
        assert_eq!(output3[i].x, items[i].get_bounds().x);
        assert_eq!(output3[i].y, items[i].get_bounds().y);
        assert_eq!(output3[i].w, items[i].get_bounds().w);
        assert_eq!(output3[i].h, items[i].get_bounds().h);
    }
}

#[test]
fn layout_row_vertical() {
    let layout = TreemapLayout::new();
    let bounds = Rect::new_from_points(0.0, 0.0, 4.0, 6.0);
    let mut items: Vec<Box<Mappable>> = vec![
        Box::new(MapItem::new_from_size_and_order(6.0, 0)),
        Box::new(MapItem::new_from_size_and_order(6.0, 0)),
        Box::new(MapItem::new_from_size_and_order(4.0, 0)),
    ];

    let output1 = vec![Rect::new_from_points(0.0, 0.0, 4.0, 1.5)];
    let output2 = vec![
        Rect::new_from_points(0.0, 0.0, 2.0, 3.0),
        Rect::new_from_points(2.0, 0.0, 2.0, 3.0),
    ];
    let output3 = vec![
        Rect::new_from_points(0.0, 0.0, 1.5, 4.0),
        Rect::new_from_points(1.5, 0.0, 1.5, 4.0),
        Rect::new_from_points(3.0, 0.0, 1.0, 4.0),
    ];

    let rect1 = layout.layout_row(&mut items, 0, 1, &bounds);

    assert_eq!(0.0, rect1.x);
    assert_eq!(1.5, rect1.y);
    assert_eq!(4.0, rect1.w);
    assert_eq!(4.5, rect1.h);

    assert_eq!(output1[0].x, items[0].get_bounds().x);
    assert_eq!(output1[0].y, items[0].get_bounds().y);
    assert_eq!(output1[0].w, items[0].get_bounds().w);
    assert_eq!(output1[0].h, items[0].get_bounds().h);

    let rect2 = layout.layout_row(&mut items, 0, 2, &bounds);

    assert_eq!(0.0, rect2.x);
    assert_eq!(3.0, rect2.y);
    assert_eq!(4.0, rect2.w);
    assert_eq!(3.0, rect2.h);

    for i in 0..2 {
        assert_eq!(output2[i].x, items[i].get_bounds().x);
        assert_eq!(output2[i].y, items[i].get_bounds().y);
        assert_eq!(output2[i].w, items[i].get_bounds().w);
        assert_eq!(output2[i].h, items[i].get_bounds().h);
    }

    let rect3 = layout.layout_row(&mut items, 0, 3, &bounds);

    assert_eq!(0.0, rect3.x);
    assert_eq!(4.0, rect3.y);
    assert_eq!(4.0, rect3.w);
    assert_eq!(2.0, rect3.h);

    for i in 0..2 {
        assert_eq!(output3[i].x, items[i].get_bounds().x);
        assert_eq!(output3[i].y, items[i].get_bounds().y);
        assert_eq!(output3[i].w, items[i].get_bounds().w);
        assert_eq!(output3[i].h, items[i].get_bounds().h);
    }
}
