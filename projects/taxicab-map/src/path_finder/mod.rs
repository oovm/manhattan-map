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
    pub fn a_star(&self, start: (isize, isize), end: (isize, isize)) -> PathFinder<T> {
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

impl<'a, T> PathFinder<'a, T> {
    pub fn with_passable<F>(mut self, passable: F) -> Self
    where
        F: Fn(isize, isize, &T) -> bool + 'static,
    {
        self.passable = Box::new(passable);
        self
    }
    pub fn with_action_cost<F>(mut self, cost: F) -> Self
    where
        F: Fn(isize, isize, &T) -> f64 + 'static,
    {
        self.action_cost = Box::new(cost);
        self
    }
    pub fn with_action_points(mut self, action_points: f64) -> Self {
        self.action_points = action_points;
        self
    }
}

impl<'a, T> Deref for PathFinder<'a, T> {
    type Target = TaxicabMap<T>;
    fn deref(&self) -> &Self::Target {
        self.map
    }
}

impl<'a, T> PathFinder<'a, T> {
    fn point_passable(&self, x: isize, y: isize) -> Option<f64> {
        let v = self.get_point(x, y)?;
        if (self.passable)(x, y, v) { Some((self.action_cost)(x, y, v)) } else { None }
    }
    pub fn neighbors(&self, x: isize, y: isize, action_cost: f64) -> Vec<(isize, isize, f64)> {
        let mut out = Vec::with_capacity(4);
        for (x, y) in self.map.points_nearby(x, y) {
            match self.point_passable(x, y) {
                Some(cost) => out.push((x, y, cost + action_cost)),
                None => (),
            }
        }
        out
    }
    fn distance(&self, (x, y): (isize, isize)) -> f64 {
        let (dx, dy) = (x - self.end.0, y - self.end.1);
        // no need sqrt here because we only need compare
        (dx * dx + dy * dy) as f64
    }
    /// A* algorithm
    pub fn solve_path(&mut self) -> Option<Vec<(isize, isize)>> {
        // fast fail if end is not passable
        if self.point_passable(self.end.0, self.end.1).is_none() {
            return None;
        }
        // normal A* algorithm
        while let Some(((x, y), cost)) = self.open.pop_last() {
            // check if we reached the end
            if (x, y) == self.end {
                return Some(self.reconstruct_path((x, y)));
            }
            // add to close list
            self.close.insert((x, y), cost);
            // add neighbors to open list
            for (x, y, cost) in self.neighbors(x, y, cost) {
                if self.close.contains_key(&(x, y)) {
                    continue;
                }
                if let Some(&old_cost) = self.open.get(&(x, y)) {
                    if old_cost <= cost {
                        continue;
                    }
                }
                self.open.insert((x, y), cost);
            }
        }
        // no path found
        None
    }

    fn reconstruct_path(&self, incoming: (isize, isize)) -> Vec<(isize, isize)> {
        todo!()
    }
}
