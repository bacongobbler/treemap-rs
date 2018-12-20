use ord_subset::OrdSubsetIterExt;

/// Interface representing an object that can be placed
/// in a treemap layout.
///
/// # Properties
///
/// - size: corresponds to area in map.
/// - order: the sort order of the item.
/// - depth: the depth in hierarchy.
/// - bounds: the bounding rectangle of the item in the map.
trait Mappable {
    fn get_size(&self) -> f64;
    fn set_size(&mut self, size: f64);
    fn get_bounds(&self) -> &Rect;
    fn set_bounds_from_rect(&mut self, bounds: Rect);
    fn set_bounds_from_points(&mut self, x: f64, y: f64, w: f64, h: f64);
    fn get_order(&self) -> i32;
    fn set_order(&mut self, order: i32);
    fn get_depth(&self) -> i32;
    fn set_depth(&mut self, depth: i32);
}

/// Model object used by MapLayout to represent data for a treemap.
trait MapModel {
    /// Get the list of items in this model. It returns an array of the Mappable objects in this MapModel.
    fn get_items(&self) -> &mut [Box<Mappable>];
}

/// The interface for the treemap layout algorithm.
trait Layout {
    /// Arrange the items in the given MapModel to fill the given rectangle.
    ///
    /// # Parameters
    ///
    /// - model: The MapModel.
    /// - bounds: The bounding rectangle for the layout.
    fn layout(&mut self, model: Box<MapModel>, bounds: Rect);
}

#[derive(Debug, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
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
        let s = [self.w / self.h, self.h / self.w];
        *s.iter().ord_subset_max().unwrap()
    }
}

pub struct MapItem {
    size: f64,
    bounds: Rect,
    order: i32,
    depth: i32,
}

impl MapItem {
    pub fn new() -> MapItem {
        MapItem::new_from_size_and_order(1.0, 0)
    }

    pub fn new_from_size_and_order(size: f64, order: i32) -> MapItem {
        MapItem {
            size: size,
            bounds: Rect::new(),
            order: order,
            depth: 0,
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

    fn get_order(&self) -> i32 {
        self.order
    }

    fn set_order(&mut self, order: i32) {
        self.order = order;
    }

    fn get_depth(&self) -> i32 {
        self.depth
    }

    fn set_depth(&mut self, depth: i32) {
        self.depth = depth;
    }
}

struct TreemapLayout {
    mid: i32,
}

impl TreemapLayout {
    pub fn layout_items(&mut self, items: &[Box<Mappable>], bounds: Rect) {
        let sorted_items = sort_descending(items);
        let end = (items.len() - 1) as i32;
        self.layout_items_at(sorted_items, 0, end, bounds);
    }
    pub fn layout_items_at(&mut self, items: &[Box<Mappable>], start: i32, end: i32, bounds: Rect) {
    }
}

impl Layout for TreemapLayout {
    fn layout(&mut self, model: Box<MapModel>, bounds: Rect) {
        self.layout_items(model.get_items(), bounds)
    }
}

fn sort_descending(items: &[Box<Mappable>]) -> &[Box<Mappable>] {
    items
}