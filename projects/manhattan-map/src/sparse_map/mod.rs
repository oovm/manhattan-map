use crate::{point::AxialPoint, Direction, HPoint, WPoint};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{btree_map::Iter, BTreeMap, BTreeSet},
};

pub mod action_field;
pub mod path_finder;

/// A sparse hexagon map, if your map size will grow, or most areas will be blank, this is a better choice.
pub struct HexagonMap<T> {
    sparse: BTreeMap<AxialPoint, T>,
}

impl<T: Default> HexagonMap<T> {
    pub fn circle(diameter: usize) -> Self {
        let mut map = BTreeMap::new();
        for x in 0..diameter {
            for y in 0..diameter {
                let point = AxialPoint::new(x as isize, y as isize);
                map.insert(point, Default::default());
            }
        }
        Self { sparse: map }
    }
    pub fn rhombus(width: usize, height: usize) -> Self {
        let mut map = BTreeMap::new();
        for x in 0..width {
            for y in 0..height {
                map.insert(AxialPoint::new(x as isize, y as isize), Default::default());
            }
        }
        Self { sparse: map }
    }
    /// Create a width first hexagon map.
    ///
    /// # Arguments
    ///
    /// * `width`: row count
    /// * `height`: column count
    /// * `odd_left`: Fill the extra line at left if width is odd.
    ///
    /// returns: HexagonMap<T>
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexagon_map::HexagonMap;
    /// let map = HexagonMap::<u8>::width_first(5, 5, true);
    /// assert_eq!(map.count_points(), 10)
    /// ```
    pub fn width_first(rows: usize, columns: usize, odd_left: bool) -> Self {
        let mut map = BTreeMap::new();
        for x in 0..rows {
            for y in 0..columns {
                let point = match rows % 2 {
                    1 if odd_left => WPoint::new(x as isize - 1, y as isize),
                    _ => WPoint::new(x as isize, y as isize),
                };
                map.insert(point.into(), Default::default());
            }
        }
        Self { sparse: map }
    }
    pub fn height_first(rows: usize, columns: usize, odd_up: bool) -> Self {
        let mut map = BTreeMap::new();
        for y in 0..columns {
            for x in 0..rows {
                let point = match columns % 2 {
                    1 if odd_up => HPoint::new(x as isize, y as isize - 1),
                    _ => HPoint::new(x as isize, y as isize),
                };
                map.insert(point.into(), Default::default());
            }
        }
        Self { sparse: map }
    }
}

impl<T> HexagonMap<T> {
    /// Get the value at a point, if it exists.
    pub fn get_point(&self, point: AxialPoint) -> Option<&T> {
        self.sparse.get(&point)
    }
    /// Add a point to the map, if it already exists, return the old value.
    pub fn add_point(&mut self, point: AxialPoint, value: T) -> Option<T> {
        self.sparse.insert(point, value)
    }
    /// Get a mutable reference to a point, if it exists.
    pub fn mut_point(&mut self, point: AxialPoint) -> Option<&mut T> {
        self.sparse.get_mut(&point)
    }
    /// Remove a point from the map, if it exists, return the old value.
    pub fn remove_point(&mut self, point: AxialPoint) -> Option<T> {
        self.sparse.remove(&point)
    }
    /// Count all defined points in the map.
    pub fn count_points(&self) -> usize {
        self.sparse.len()
    }
    /// Find at most 6 points that are exists and adjacent to a point.
    pub fn nearby_points(&self, from: &AxialPoint) -> Vec<AxialPoint> {
        from.nearby().into_iter().filter(|p| self.sparse.contains_key(p)).collect()
    }
    /// Find all points that are within a certain distance of a point.
    pub fn around_points(&self, from: &AxialPoint, distance: usize) -> Vec<AxialPoint> {
        match distance {
            0 => vec![*from],
            1 => self.nearby_points(from),
            // TODO: optimize this
            _ => {
                let mut points = vec![];
                for point in self.nearby_points(from) {
                    points.extend(self.around_points(&point, distance - 1));
                }
                points
            }
        }
    }
    pub fn points(&self) -> impl Iterator<Item = &AxialPoint> {
        self.sparse.keys()
    }
}

impl<'i, T> IntoIterator for &'i HexagonMap<T> {
    type Item = (&'i AxialPoint, &'i T);
    type IntoIter = Iter<'i, AxialPoint, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.sparse.iter()
    }
}
