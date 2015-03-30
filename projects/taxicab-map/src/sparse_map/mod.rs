use crate::{point::Point, Direction};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{btree_map::Iter, BTreeMap, BTreeSet},
};
use std::mem::swap;
use ndarray::Array2;

pub mod action_field;
pub mod path_finder;

/// A dense manhattan map, if your map size will grow, or most areas will be blank, this is a better choice.
pub struct TaxicabMap<T> {
    dense: Array2<T>,
    cycle_x: bool,
    cycle_y: bool,
}

impl<T: Clone> TaxicabMap<T> {
    pub fn square(width: usize, fill: &T) -> Self {
        Self::rectangle(width, width)
    }
    pub fn rectangle(width: usize, height: usize, fill: &T) -> Self {
        let mut map = Array2::default((width, height));
        Self { cycle_x: false, cycle_y: false, dense: map }
    }
    pub fn with_cycle(mut self, cycle_x: bool, cycle_y: bool) -> Self {
        self.cycle_x = cycle_x;
        self.cycle_y = cycle_y;
        self
    }
    pub fn get_cycle(&self) -> (bool, bool) {
        (self.cycle_x, self.cycle_y)
    }
    pub fn set_cycle(&mut self, cycle_x: bool, cycle_y: bool) {
        self.cycle_x = cycle_x;
        self.cycle_y = cycle_y;
    }
    pub fn get_point(&self, point: Point) -> Option<&T> {
        self.dense.get(point)
    }
    pub fn set_point(&mut self, point: Point, value: T) {
        self.dense[point] = value;
    }
    pub fn extend(&mut self, direction: Direction, size: usize) {
        let (x, y) = self.dense.dim();
        let (w, h) = match direction {
            Direction::X(_) => (x + size, y),
            Direction::Y(_) => (x, y + size),
        };
        let mut new = Array2::default((w, h));
        match direction {
            Direction::X(true) => {
                for (x, y) in (0..x).cartesian_product(0..y) {
                    swap(&mut new[[x + size, y]], &mut self.dense[[x, y]]);
                }
            }
            Direction::X(false) => {
                for (x, y) in (0..x).cartesian_product(0..y) {
                    swap(&mut new[[x, y]], &mut self.dense[[x, y]]);
                }
            }
            Direction::Y(true) => {
                for (x, y) in (0..x).cartesian_product(0..y) {
                    swap(&mut new[[x, y + size]], &mut self.dense[[x, y]]);
                }
            }
            Direction::Y(false) => {
                for (x, y) in (0..x).cartesian_product(0..y) {
                    swap(&mut new[[x, y]], &mut self.dense[[x, y]]);
                }
            }
        }
        self.dense = new;
    }
}

impl<T> TaxicabMap<T> {
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
    pub fn points(&self) -> impl Iterator<Item=&Point> {
        self.dense.keys()
    }
}

impl<'i, T> IntoIterator for &'i TaxicabMap<T> {
    type Item = (&'i Point, &'i T);
    type IntoIter = Iter<'i, Point, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.dense.iter()
    }
}
