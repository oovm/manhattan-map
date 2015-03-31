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
pub mod iters;

/// A dense manhattan map, if your map size will grow, or most areas will be blank, this is a better choice.
pub struct TaxicabMap<T> {
    dense: Array2<T>,
    cycle_x: bool,
    cycle_y: bool,
    origin_x: isize,
    origin_y: isize,
}

impl<T: Clone> TaxicabMap<T> {
    pub fn square(width: usize, fill: &T) -> Self {
        Self::rectangle(width, width, fill)
    }
    pub fn rectangle(width: usize, height: usize, fill: &T) -> Self {
        let dense = Array2::from_shape_fn((width, height), |_| fill.clone());
        Self {
            dense,
            cycle_x: false,
            cycle_y: false,
            origin_x: 0,
            origin_y: 0,
        }
    }
    pub fn extend(&mut self, direction: Direction, size: usize, fill: &T) {
        let (x, y) = self.dense.dim();
        let (w, h) = match direction {
            Direction::X(_) => (x + size, y),
            Direction::Y(_) => (x, y + size),
        };
        let mut new = Array2::from_shape_fn((w, h), |_| fill.clone());
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
    pub fn shift_origin(&mut self, x: isize, y: isize) {
        self.origin_x += x;
        self.origin_y += y;
    }
    pub fn get_size(&self) -> (usize, usize) {
        self.dense.dim()
    }
    pub fn has_point(&self, point: Point) -> bool {
        let relative = absolute_to_relative(point.x, point.y, self);
        let size = self.get_size();
        relative.0 < size.0 && relative.1 < size.1
    }
    pub fn get_point(&self, point: Point) -> Option<&T> {
        let relative = absolute_to_relative(point.x, point.y, self);
        self.dense.get(relative)
    }
    pub fn mut_point(&mut self, point: Point) -> Option<&mut T> {
        let relative = absolute_to_relative(point.x, point.y, self);
        self.dense.get_mut(relative)
    }
    pub fn set_point(&mut self, point: Point, value: T) -> bool {
        match self.mut_point(point) {
            Some(v) => {
                *v = value;
                true
            }
            None => false,
        }
    }
    /// Count all defined points in the map.
    pub fn count_points(&self) -> usize {
        self.dense.len()
    }
}

fn absolute_to_relative<T>(x: isize, y: isize, map: &TaxicabMap<T>) -> (usize, usize) {
    let (w, h) = map.get_size();
    let (x, y) = if map.cycle_x {
        (x % w as isize, y)
    } else {
        (x - map.origin_x, y)
    };
    let (x, y) = if map.cycle_y {
        (x, y % h as isize)
    } else {
        (x, y - map.origin_y)
    };
    let x = if x < 0 { w as isize + x } else { x } as usize;
    let y = if y < 0 { h as isize + y } else { y } as usize;
    (x, y)
}

impl<T> TaxicabMap<T> {
    /// Find at most 6 points that are exists and adjacent to a point.
    pub fn nearby_points(&self, from: &Point) -> Vec<Point> {
        from.nearby().into_iter().filter(|p| self.has_point(*p)).collect()
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
}
