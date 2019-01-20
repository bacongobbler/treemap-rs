
/// Interface representing an object that can be placed
/// in a treemap layout.
///
/// # Properties
///
/// - size: corresponds to area in map.
/// - order: the sort order of the item.
/// - depth: the depth in hierarchy.
/// - bounds: the bounding rectangle of the item in the map.
pub trait Mappable {
    fn get_size(&self) -> f64;
    fn set_size(&mut self, size: f64);
    fn get_bounds(&self) -> &Rect;
    fn set_bounds_from_rect(&mut self, bounds: Rect);
    fn set_bounds_from_points(&mut self, x: f64, y: f64, w: f64, h: f64);
}

/// Model object used by MapLayout to represent data for a treemap.
pub trait MapModel {
    /// Get the list of items in this model. It returns an array of the Mappable objects in this MapModel.
    fn get_items(&self) -> Vec<Box<Mappable>>;
}

/// The interface for the treemap layout algorithm.
pub trait Layout {
    /// Arrange the items in the given MapModel to fill the given rectangle.
    ///
    /// # Parameters
    ///
    /// - model: The MapModel.
    /// - bounds: The bounding rectangle for the layout.
    fn layout(&mut self, model: &mut Box<MapModel>, bounds: Rect);
}

#[derive(Debug, PartialEq, Copy)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Clone for Rect {
    fn clone(&self) -> Rect {
        *self
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

    pub fn new_from_rect(rect: Rect) -> Rect {
        Rect::new_from_points(rect.x, rect.y, rect.w, rect.h)
    }

    pub fn aspect_ratio(&self) -> f64 {
        (self.w / self.h).max(self.h / self.w)
    }
}

#[derive(Copy)]
pub struct MapItem {
    size: f64,
    bounds: Rect,
}

impl Clone for MapItem {
    fn clone(&self) -> MapItem {
        *self
    }
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

    fn set_size(&mut self, size: f64) {
        self.size = size;
    }

    fn get_bounds(&self) -> &Rect {
        &self.bounds
    }

    fn set_bounds_from_rect(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn set_bounds_from_points(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.bounds.x = x;
        self.bounds.y = y;
        self.bounds.w = w;
        self.bounds.h = h;
    }
}

pub struct TreemapLayout {}

impl TreemapLayout {
    pub fn new() -> TreemapLayout {
        TreemapLayout {}
    }

    pub fn layout_items(&mut self, mut items: &mut Vec<Box<Mappable>>, bounds: Rect) {
        sort_descending(&mut items);
        let end = items.len() - 1;
        self.layout_items_at(&mut items, 0, end, bounds);
    }

    pub fn layout_items_at(
        &mut self,
        mut items: &mut Vec<Box<Mappable>>,
        start: usize,
        end: usize,
        bounds: Rect,
    ) {
        if start > end {
            return;
        }
        if end - start < 2 {
            self.layout_row(&mut items, start, end, &bounds);
            return;
        }

        let x = bounds.x;
        let y = bounds.y;
        let w = bounds.w;
        let h = bounds.h;

        let total = self.total_item_size_with_range(items, start, end);
        let mut mid = start;
        let a = items[start].get_size() / total;
        let mut b = a;

        if w < h {
            // height/width
            while mid <= end {
                let aspect = norm_aspect(h, w, a, b);
                let q = items[mid].get_size() / total;
                if norm_aspect(h, w, a, b + q) > aspect {
                    break;
                }
                mid += 1;
                b += q;
            }
            self.layout_row(
                &mut items,
                start,
                mid,
                &Rect::new_from_points(x, y, w, h * b),
            );
            self.layout_items_at(
                &mut items,
                mid + 1,
                end,
                Rect::new_from_points(x, y + h * b, w, h * (1.0 - b)),
            );
        } else {
            // width/height
            while mid <= end {
                let aspect = norm_aspect(w, h, a, b);
                let q = items[mid].get_size() / total;
                if norm_aspect(w, h, a, b + q) > aspect {
                    break;
                }
                mid += 1;
                b += q;
            }
            self.layout_row(
                &mut items,
                start,
                mid,
                &Rect::new_from_points(x, y, w * b, h),
            );
            self.layout_items_at(
                &mut items,
                mid + 1,
                end,
                Rect::new_from_points(x + w * b, y, w * (1.0 - b), h),
            );
        }
    }

    pub fn layout_row(
        &self,
        items: &mut Vec<Box<Mappable>>,
        start: usize,
        end: usize,
        bounds: &Rect,
    ) {
        let is_horizontal = bounds.w > bounds.h;
        let total = self.total_item_size_with_range(&items, start, end + 1);
        let mut a = 0.0;

        for i in start..end + 1 {
            let mut r = Rect::new();
            let b = items[i].get_size() / total;

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
            items[i].set_bounds_from_rect(r);
            a += b;
        }
    }

    pub fn total_item_size(&self, items: &Vec<Box<Mappable>>) -> f64 {
        let mut sum = 0.0;
        for item in items {
            sum += item.get_size();
        }
        sum
    }

    pub fn total_item_size_with_range(
        &self,
        items: &Vec<Box<Mappable>>,
        start: usize,
        end: usize,
    ) -> f64 {
        let mut sum = 0.0;
        for i in start..end {
            sum += items[i].get_size();
        }
        sum
    }
}

impl Layout for TreemapLayout {
    fn layout(&mut self, model: &mut Box<MapModel>, bounds: Rect) {
        let mut items = model.get_items();
        self.layout_items(&mut items, bounds)
    }
}

pub fn sort_descending(items: &mut Vec<Box<Mappable>>) {
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
