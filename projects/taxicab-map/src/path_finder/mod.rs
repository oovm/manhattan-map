use crate::TaxicabMap;
use std::{collections::BTreeMap, ops::Deref};

pub struct PathFinder<'a, T> {
    map: &'a TaxicabMap<T>,
    start: (isize, isize),
    end: (isize, isize),
    open: BTreeMap<(isize, isize), f64>,
    close: BTreeMap<(isize, isize), f64>,
    passable: Box<dyn Fn(isize, isize, &T) -> bool>,
    action_points: f64,
    action_cost: Box<dyn Fn(isize, isize, &T) -> f64>,
}

impl<T> TaxicabMap<T> {
    pub fn path_finder(&self, start: (isize, isize), end: (isize, isize)) -> PathFinder<T> {
        let mut open = BTreeMap::new();
        open.insert(start, 0.0);
        PathFinder {
            map: self,
            start,
            end,
            open,
            close: Default::default(),
            passable: Box::new(|_, _, _| true),
            action_points: f64::INFINITY,
            action_cost: Box::new(|_, _, _| 1.0),
        }
    }
}

impl<'a, T> Deref for PathFinder<'a, T> {
    type Target = TaxicabMap<T>;
    fn deref(&self) -> &Self::Target {
        self.map
    }
}

impl<'a, T> PathFinder<'a, T> {
    fn passable_point(&self, x: isize, y: isize) -> Option<f64> {
        let v = self.get_point(x, y)?;
        if (self.passable)(x, y, v) { Some((self.action_cost)(x, y, v)) } else { None }
    }

    pub fn neighbors(&self, x: isize, y: isize) -> Vec<(isize, isize, &T)> {
        self.map.points_nearby(x, y).filter_map(|(x, y)| self.get_point(x, y).map(|v| (x, y, v))).collect()
    }
}
