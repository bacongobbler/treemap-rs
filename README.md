# Treemap

Implements the Squarified Treemap algorithm published by Mark Bruls, Kees Huizing, and Jarke J. van Wijk.

The Squarified Treemap algorithm paper can be found here: <https://www.win.tue.nl/~vanwijk/stm.pdf>

## Uses

Suppose we have a rectangle with a width of 6 and a height of 4, and furthermore suppose that this rectangle must be subdivided in seven rectangles with areas 6, 6, 4, 3, 2, 2, and 1. The standard treemap algorithm uses a simple approach: The rectangle is subdivided either horizontally or vertically. Thin rectangles emerge, with aspect ratios of 16 and 36, respectively.

In other words, it'll look something like this:

```text
+------+------+----+---+--+-+
|      |      |    |   |  | |
|      |      |    |   |  | |
|   6  |   6  |  4 | 3 | 2|1|
|      |      |    |   |  | |
+------+------+----+---+--+-+
```

The Squarified Treemap algorithm tesselates a rectangle recursively into rectangles, such that their aspect ratios approach 1 as close as possible.

```text
+-------------+-----+-----+--+
|             |  2  |  2  | 1|
|      6      +-----+-+---+--|
|-------------+       |      |
|             |       |      |
|      6      |    4  |   3  |
+-------------+-------+------+
```

This can be useful for a variety of purposes:

- visualizing hierarchal structures, such as showing how much space each directory uses in a file drive
- generating a floor map given an area on how each room should be subdivided (bathrooms would need a smaller amount of space than a living room, for example)

## Example

This example will tesselate a rectangle with a width of 6 and a height of 4 with seven rectangles with areas 6, 6, 4, 3, 2, 2, and 1, then display each rectangle's top-left corner's x and y position within the larger rectangle (the bounds), as well as their respective height and width.

To start, generate a new project:

```console
$ cargo new --bin treemap-example
     Created binary (application) `treemap-example` package
```

Add `treemap` to Cargo.toml:

```toml
[dependencies]
treemap = "0.2.0"
```

Then, in `src/main.rs`:

```rust
extern crate treemap;

use treemap::{MapItem, Mappable, Rect, TreemapLayout};

fn main() {
    let mut layout = TreemapLayout::new();
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

    layout.layout_items(&mut items, bounds);

    for item in items {
        let item_bounds = item.get_bounds();
        println!("x={} y={} w={} h={}", i, item_bounds.x, item_bounds.y, item_bounds.w, item_bounds.h);
        println!("------");
    }
}
```
