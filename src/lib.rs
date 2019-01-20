/// Interface representing an object that can be placed
/// in a treemap layout.
///
/// # Properties
///
/// - size: corresponds to area in map.
/// - bounds: the bounding rectangle of the item in the map.
pub trait Mappable {
    fn get_size(&self) -> f64;
    fn get_bounds(&self) -> &Rect;
    fn set_bounds_from_rect(&mut self, bounds: Rect);
}

impl Mappable for Box<dyn Mappable> {
    fn get_size(&self) -> f64 {
        (**self).get_size()
    }
    fn get_bounds(&self) -> &Rect {
        (**self).get_bounds()
    }
    fn set_bounds_from_rect(&mut self, bounds: Rect) {
        (**self).set_bounds_from_rect(bounds)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Default for Rect {
    fn default() -> Self {
        Self::new()
    }
}

impl Rect {
    pub fn new() -> Rect {
        Rect {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        }
    }

    pub fn new_from_points(x: f64, y: f64, w: f64, h: f64) -> Rect {
        Rect {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    pub fn aspect_ratio(&self) -> f64 {
        if self.h != 0. && self.w != 0. {
            (self.w / self.h).max(self.h / self.w)
        } else {
            0.
        }
    }
}

#[derive(Copy, Clone)]
pub struct MapItem {
    size: f64,
    bounds: Rect,
}

impl MapItem {
    pub fn new() -> MapItem {
        MapItem::new_with_size(1.0)
    }

    pub fn new_with_size(size: f64) -> MapItem {
        MapItem {
            size: size,
            bounds: Rect::new(),
        }
    }
}

impl Mappable for MapItem {
    fn get_size(&self) -> f64 {
        self.size
    }

    fn get_bounds(&self) -> &Rect {
        &self.bounds
    }

    fn set_bounds_from_rect(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }
}

pub struct TreemapLayout {}

impl TreemapLayout {
    pub fn new() -> Self {
        Self {}
    }

    pub fn layout_items<T: Mappable>(&self, items: &mut [T], bounds: Rect) {
        sort_descending(items);
        let end = items.len();
        self.layout_items_at(items, 0, end, bounds);
    }

    fn layout_items_at<T: Mappable>(
        &self,
        items: &mut [T],
        start: usize,
        end: usize,
        bounds: Rect,
    ) {
        if end - start <= 2 {
            self.layout_row(&mut items[start..end], bounds);
            return;
        }

        let x = bounds.x;
        let y = bounds.y;
        let w = bounds.w;
        let h = bounds.h;

        let total = self.total_item_size(&items[start..end - 1]);
        let mut mid = start;
        let a = items[start].get_size() / total;
        let mut b = a;

        let (mid, rect, rect2) = if w < h {
            // height/width
            while mid < end {
                let aspect = norm_aspect(h, w, a, b);
                let q = items[mid].get_size() / total;
                if norm_aspect(h, w, a, b + q) > aspect {
                    break;
                }
                mid += 1;
                b += q;
            }
            (
                mid,
                Rect::new_from_points(x, y, w, h * b),
                Rect::new_from_points(x, y + h * b, w, h * (1.0 - b)),
            )
        } else {
            // width/height
            while mid < end {
                let aspect = norm_aspect(w, h, a, b);
                let q = items[mid].get_size() / total;
                if norm_aspect(w, h, a, b + q) > aspect {
                    break;
                }
                mid += 1;
                b += q;
            }
            (
                mid,
                Rect::new_from_points(x, y, w * b, h),
                Rect::new_from_points(x + w * b, y, w * (1.0 - b), h),
            )
        };
        if mid < end {
            self.layout_row(&mut items[start..=mid], rect);
            if mid + 1 < end {
                self.layout_items_at(items, mid + 1, end, rect2);
            }
        }
    }

    fn layout_row<T: Mappable>(&self, items: &mut [T], bounds: Rect) {
        let is_horizontal = bounds.w > bounds.h;
        let total = self.total_item_size(items);
        let mut a = 0.0;

        for item in items {
            let mut r = Rect::new();
            let b = item.get_size() / total;

            if is_horizontal {
                r.x = bounds.x + bounds.w * a;
                r.w = bounds.w * b;
                r.y = bounds.y;
                r.h = bounds.h;
            } else {
                r.x = bounds.x;
                r.w = bounds.w;
                r.y = bounds.y + bounds.h * a;
                r.h = bounds.h * b;
            }
            item.set_bounds_from_rect(r);
            a += b;
        }
    }

    fn total_item_size<T: Mappable>(&self, items: &[T]) -> f64 {
        items.iter().map(|i| i.get_size()).sum()
    }
}

fn sort_descending<T: Mappable>(items: &mut [T]) {
    items.sort_by(|a, b| b.get_size().partial_cmp(&a.get_size()).unwrap());
}

fn norm_aspect(big: f64, small: f64, a: f64, b: f64) -> f64 {
    let x = aspect(big, small, a, b);
    if x < 1.0 {
        return 1.0 / x;
    }
    x
}

fn aspect(big: f64, small: f64, a: f64, b: f64) -> f64 {
    return (big * b) / (small * a / b);
}


#[test]
fn test_sort_descending() {
    let input = [
        24.0, 2.0, 45.0, 20.0, 56.0, 75.0, 2.0, 56.0, 99.0, 53.0, 12.0,
    ];
    let output = [
        99.0, 75.0, 56.0, 56.0, 53.0, 45.0, 24.0, 20.0, 12.0, 2.0, 2.0,
    ];
    let mut items: Vec<Box<dyn Mappable>> = Default::default();
    for i in input.iter() {
        items.push(Box::new(MapItem::new_with_size(*i)));
    }
    sort_descending(&mut items);
    for i in 0..items.len() {
        assert_eq!(output[i], items[i].get_size());
    }
}
