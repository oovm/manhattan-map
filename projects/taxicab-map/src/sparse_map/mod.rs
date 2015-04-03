use crate::{Direction};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{btree_map::Iter, BTreeMap, BTreeSet},
};
use std::mem::swap;
use ndarray::{Array2, ArrayView2};

// pub mod action_field;
// pub mod path_finder;
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
    pub fn has_point(&self, x: isize, y: isize) -> bool {
        let (w, h) = self.get_size();
        let (i, j) = absolute_to_relative(x, y,  self.origin_x, self.origin_y, w, h, self.cycle_x, self.cycle_y);
        i < w && j < h
    }
    pub fn get_point(&self, x: isize, y: isize) -> Option<&T> {
        let (w, h) = self.get_size();
        let (i, j) = absolute_to_relative(x, y,  self.origin_x, self.origin_y, w, h, self.cycle_x, self.cycle_y);
        self.dense.get((i, j))
    }
    pub fn mut_point(&mut self, x: isize, y: isize) -> Option<&mut T> {
        let (w, h) = self.get_size();
        let (i, j) = absolute_to_relative(x, y,  self.origin_x, self.origin_y, w, h, self.cycle_x, self.cycle_y);
        self.dense.get_mut((i, j))
    }
    pub fn set_point(&mut self, x: isize, y: isize, value: T) -> bool {
        match self.mut_point(x, y) {
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


#[inline]
pub(crate) fn absolute_to_relative(x: isize, y: isize, origin_x: isize, origin_y: isize, w: usize, h: usize, cycle_x: bool, cycle_y: bool) -> (usize, usize) {
    let (w, h) = (w as isize, h as isize);
    let (mut x, mut y) = (x - origin_x, y - origin_y);
    if cycle_x {
        x = x.rem_euclid(w);
    }
    if cycle_y {
        y = y.rem_euclid(h);
    }
    (x as usize, y as usize)
}

#[inline]
pub(crate) fn relative_to_absolute(x: usize, y: usize, origin_x: isize, origin_y: isize) -> (isize, isize) {
    (x as isize + origin_x, y as isize + origin_y)
}