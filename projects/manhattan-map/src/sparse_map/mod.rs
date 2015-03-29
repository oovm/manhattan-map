use crate::{point::Point, Direction};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{btree_map::Iter, BTreeMap, BTreeSet},
};

pub mod action_field;
pub mod path_finder;

/// A sparse hexagon map, if your map size will grow, or most areas will be blank, this is a better choice.
pub struct HexagonMap<T> {
    dense: BTreeMap<Point, T>,
}

impl<T: Default> HexagonMap<T> {
    pub fn circle(diameter: usize) -> Self {
        let mut map = BTreeMap::new();
        for x in 0..diameter {
            for y in 0..diameter {
                let point = Point::new(x as isize, y as isize);
                map.insert(point, Default::default());
            }
        }
        Self { dense: map }
    }
    pub fn rhombus(width: usize, height: usize) -> Self {
        let mut map = BTreeMap::new();
        for x in 0..width {
            for y in 0..height {
                map.insert(Point::new(x as isize, y as isize), Default::default());
            }
        }
        Self { dense: map }
    }
}

impl<T> HexagonMap<T> {
    /// Get the value at a point, if it exists.
    pub fn get_point(&self, point: Point) -> Option<&T> {
        self.dense.get(&point)
    }
    /// Add a point to the map, if it already exists, return the old value.
    pub fn add_point(&mut self, point: Point, value: T) -> Option<T> {
        self.dense.insert(point, value)
    }
    /// Get a mutable reference to a point, if it exists.
    pub fn mut_point(&mut self, point: Point) -> Option<&mut T> {
        self.dense.get_mut(&point)
    }
    /// Remove a point from the map, if it exists, return the old value.
    pub fn remove_point(&mut self, point: Point) -> Option<T> {
        self.dense.remove(&point)
    }
    /// Count all defined points in the map.
    pub fn count_points(&self) -> usize {
        self.dense.len()
    }
    /// Find at most 6 points that are exists and adjacent to a point.
    pub fn nearby_points(&self, from: &Point) -> Vec<Point> {
        from.nearby().into_iter().filter(|p| self.dense.contains_key(p)).collect()
    }
    /// Find all points that are within a certain distance of a point.
    pub fn around_points(&self, from: &Point, distance: usize) -> Vec<Point> {
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
    pub fn points(&self) -> impl Iterator<Item = &Point> {
        self.dense.keys()
    }
}

impl<'i, T> IntoIterator for &'i HexagonMap<T> {
    type Item = (&'i Point, &'i T);
    type IntoIter = Iter<'i, Point, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.dense.iter()
    }
}
