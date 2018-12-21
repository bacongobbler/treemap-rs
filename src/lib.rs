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
    fn layout(&mut self, model: &MapModel, bounds: Rect);
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
    mid: usize,
}

impl TreemapLayout {
    pub fn layout_items(&mut self, items: &mut [Box<Mappable>], bounds: Rect) {
        let sorted_items = sort_descending(items);
        let end = sorted_items.len() - 1;
        self.layout_items_at(sorted_items, 0, end, bounds);
    }

    pub fn layout_items_at(&mut self, items: &mut [Box<Mappable>], start: usize, end: usize, bounds: Rect) {
        if start > end {
            return;
        }
        if start == end {
            items[start].set_bounds_from_points(bounds.x, bounds.y, bounds.w, bounds.h);
        }

        self.mid = start;
        while self.mid < end {
            if self.highest_aspect(items, start, self.mid, &bounds)
                > self.highest_aspect(items, start, self.mid + 1, &bounds)
            {
                self.mid += 1;
            } else {
                let new_bounds = self.layout_row(items, start, self.mid, &bounds);
                self.layout_items_at(items, self.mid + 1, end, new_bounds);
            }
        }
    }

    fn highest_aspect(&self, items: &mut [Box<Mappable>], start: usize, end: usize, bounds: &Rect) -> f64 {
        self.layout_row(items, start, end, bounds);
        let mut max = std::f64::MIN;
        for i in start..end+1 {
            let aspect_ratio = items[i].get_bounds().aspect_ratio();
            if aspect_ratio > max {
                max = aspect_ratio;
            }
        }
        max
    }

    pub fn layout_row(&self, items: &mut [Box<Mappable>], start: usize, end: usize, bounds: &Rect) -> Rect {
        let is_horizontal = bounds.w > bounds.h;
        let total = bounds.w * bounds.h;
        let row_size = self.total_item_size_with_range(items, start, end);
        let row_ratio = row_size / total;
        let mut offset = 0.0;

        for i in start..end+1 {
            let mut r = Rect::new();
            let ratio = items[i].get_size() / row_size;

            if is_horizontal {
                r.x = bounds.x;
                r.w = bounds.w * row_ratio;
                r.y = bounds.y + bounds.h * offset;
                r.h = bounds.h * ratio;
            } else {
                r.x = bounds.x + bounds.w * offset;
                r.w = bounds.w * ratio;
                r.y = bounds.y;
                r.h = bounds.h * row_ratio;
            }
            items[i].set_bounds_from_rect(r);
            offset += ratio;
        }

        if is_horizontal {
            return Rect {
                x: bounds.x + bounds.w * row_ratio,
                y: bounds.y,
                w: bounds.w - bounds.w * row_ratio,
                h: bounds.h,
            };
        }
        return Rect {
            x: bounds.x,
            y: bounds.y + bounds.h * row_ratio,
            w: bounds.w,
            h: bounds.h - bounds.h * row_ratio,
        };
    }

    pub fn total_item_size(&self, items: &[Box<Mappable>]) -> f64 {
        self.total_item_size_with_range(items, 0, items.len())
    }

    pub fn total_item_size_with_range(&self, items: &[Box<Mappable>], start: usize, end: usize) -> f64 {
        let mut sum = 0.0;
        for i in start..end {
            sum += items[i].get_size();
        }
        sum
    }
}

impl Layout for TreemapLayout {
    fn layout(&mut self, model: &MapModel, bounds: Rect) {
        self.layout_items(model.get_items(), bounds)
    }
}

fn sort_descending(items: &mut [Box<Mappable>]) -> &mut [Box<Mappable>] {
    if items.len() == 0 {
        return items;
    }
    quick_sort_desc(items, 0, items.len() - 1)
}

fn quick_sort_desc(
    input: &mut [Box<Mappable>],
    lower_index: usize,
    higher_index: usize,
) -> &mut [Box<Mappable>] {
    let mut i = lower_index;
    let mut j = higher_index;
    let pivot: f64 = input[lower_index + (higher_index - lower_index) / 2].get_size();
    while i <= j {
        // In each iteration, we will identify a number from left side which
        // is greater then the pivot value, and also we will identify a number
        // from right side which is less then the pivot value. Once the search
        // is done, then we exchange both numbers.
        while input[i].get_size() > pivot {
            i += 1;
        }
        while input[j].get_size() < pivot {
            j -= 1;
        }
        if i <= j {
            input.swap(i, j);
            // move index to next position on both sides
            i += 1;
            j -= 1;
        }
    }
    input
}
