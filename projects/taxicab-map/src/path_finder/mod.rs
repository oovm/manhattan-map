use crate::{Joint, TaxicabMap};
use ordered_float::OrderedFloat;
use pathfinding::prelude::astar;
use std::collections::VecDeque;

/// A* path finder on a taxicab map.
pub struct PathFinder<'a, T> {
    map: &'a TaxicabMap<T>,
    start: (isize, isize),
    end: (isize, isize),
    passable: Box<dyn Fn(isize, isize, &T) -> bool>,
    action_cost: Box<dyn Fn(isize, isize, &T) -> f64>,
}

impl<T> TaxicabMap<T> {
    /// Create a path finder.
    pub fn path_finder(&self, start: (isize, isize), end: (isize, isize)) -> PathFinder<T> {
        let mut open = VecDeque::new();
        open.push_back((0.0, start));
        PathFinder { map: self, start, end, passable: Box::new(|_, _, _| true), action_cost: Box::new(|_, _, _| 1.0) }
    }
}

impl<'a, T> PathFinder<'a, T> {
    /// Set the passable function.
    pub fn with_passable<F>(mut self, passable: F) -> Self
    where
        F: Fn(isize, isize, &T) -> bool + 'static,
    {
        self.passable = Box::new(passable);
        self
    }
    /// Set the action cost function.
    pub fn with_action_cost<F>(mut self, cost: F) -> Self
    where
        F: Fn(isize, isize, &T) -> f64 + 'static,
    {
        self.action_cost = Box::new(cost);
        self
    }
}

impl<'a, T> PathFinder<'a, T> {
    fn point_passable(&self, x: isize, y: isize) -> Option<OrderedFloat<f64>> {
        let v = self.map.get_point(x, y)?;
        if (self.passable)(x, y, v) {
            let cost = (self.action_cost)(x, y, v);
            Some(OrderedFloat(cost))
        }
        else {
            None
        }
    }
    fn neighbors(&self, (x, y): (isize, isize)) -> Vec<((isize, isize), OrderedFloat<f64>)> {
        let mut out = Vec::with_capacity(4);
        for (x, y) in self.map.points_nearby(x, y) {
            match self.point_passable(x, y) {
                Some(cost) => out.push(((x, y), cost)),
                None => (),
            }
        }
        out
    }
    // taxicab distance as heuristic
    fn heuristic(&self, (x, y): (isize, isize)) -> OrderedFloat<f64> {
        let dx = (x - self.end.0).abs();
        let dy = (y - self.end.1).abs();
        OrderedFloat((dx + dy) as f64)
    }
    /// A* algorithm
    pub fn solve_path(self) -> (Vec<(isize, isize)>, f64) {
        astar(&self.start, |p| self.neighbors(*p), |p| self.heuristic(*p), |(x, y)| self.end == (*x, *y))
            .map(|(path, cost)| (path, cost.0))
            .unwrap_or((vec![], f64::INFINITY))
    }
    /// Solve by path and convert to joints
    pub fn solve_joint(self) -> (Vec<Joint>, f64) {
        let mut out = vec![];
        let (path, cost) = self.solve_path();
        if path.is_empty() {
            return (vec![], f64::INFINITY);
        }
        for (from, to) in path.iter().zip(path.iter().skip(1)) {
            out.push(Joint::from_point(*from, *to))
        }
        (out, cost)
    }
}
