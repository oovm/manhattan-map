use super::*;

/// A* path finder on a hexagon map.
pub struct PathFinder<'a, T> {
    map: &'a TaxicabMap<T>,
    start: Point,
    end: Point,
    open: BTreeSet<Point>,
    close: BTreeSet<Point>,
    passable: Box<dyn Fn(&Point, &T) -> bool>,
    action_point: Option<f64>,
    action_cost: Box<dyn Fn(&Point, &T) -> f64>,
}

impl<T> TaxicabMap<T> {
    /// Set the passable function.
    ///
    /// # Arguments
    ///
    /// * `passable`:  A function that returns true if the point is passable.
    ///
    /// returns: PathFinder<T>
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexagon_map::HexagonMap;
    /// ```
    pub fn path_finder(&self, start: &Point, end: &Point) -> PathFinder<T> {
        let mut open = BTreeSet::new();
        open.insert(start.clone());
        PathFinder {
            map: self,
            start: start.clone(),
            end: end.clone(),
            open,
            close: Default::default(),
            passable: Box::new(|_, _| true),
            action_point: None,
            action_cost: Box::new(|_, _| 0.0),
        }
    }
}

impl<'a, T> PathFinder<'a, T> {
    /// Set the passable function.
    ///
    /// # Arguments
    ///
    /// * `passable`:  A function that returns true if the point is passable.
    ///
    /// returns: PathFinder<T>
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexagon_map::HexagonMap;
    /// ```
    pub fn with_passable<F>(mut self, passable: F) -> Self
    where
        F: Fn(&Point, &T) -> bool + 'static,
    {
        self.passable = Box::new(passable);
        self
    }
    /// Set the passable function.
    ///
    /// # Arguments
    ///
    /// * `passable`:  A function that returns true if the point is passable.
    ///
    /// returns: PathFinder<T>
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexagon_map::HexagonMap;
    /// ```
    pub fn with_action(mut self, action: f64) -> Self {
        if action.is_sign_negative() {
            self.action_point = None;
        }
        else {
            self.action_point = Some(action);
        }
        self
    }
    /// Set the passable function.
    ///
    /// # Arguments
    ///
    /// * `passable`:  A function that returns true if the point is passable.
    ///
    /// returns: PathFinder<T>
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexagon_map::HexagonMap;
    /// ```
    pub fn with_cost<F>(mut self, cost: F) -> Self
    where
        F: Fn(&Point, &T) -> f64 + 'static,
    {
        self.action_point = Some(0.0);
        self.action_cost = Box::new(cost);
        self
    }
    pub fn get_point(&self, point: &Point) -> Option<&T> {
        self.map.dense.get(point)
    }
    pub fn has_point(&self, point: &Point) -> bool {
        self.map.dense.contains_key(point)
    }
    pub fn distance_to_start(&self, point: &Point) -> usize {
        self.start.manhattan_distance(point)
    }
    pub fn distance_to_end(&self, point: &Point) -> usize {
        self.end.manhattan_distance(point)
    }
    /// Get all passable neighbors from a point
    pub fn neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = vec![];
        for direction in Direction::all() {
            if let Some(target) = self.map.dense.get(&point.go(direction)) {
                if (self.passable)(point, target) {
                    neighbors.push(point.go(direction));
                }
            }
        }
        neighbors
    }
    fn fast_reject(&self) -> bool {
        !self.has_point(&self.start) || !self.has_point(&self.end)
    }
    pub fn solve_path(mut self) -> Option<Vec<Point>> {
        if self.fast_reject() {
            return None;
        }
        while let Some(current) = self.open.pop_first() {
            if current == self.end {
                return Some(self.reconstruct_path(&current));
            }
            self.close.insert(current.clone());
            for neighbor in self.neighbors(&current) {
                if self.close.contains(&neighbor) {
                    continue;
                }
                let tentative_g_score = self.distance_to_start(&current) + 1;
                if !self.open.contains(&neighbor) {
                    self.open.insert(neighbor);
                }
                else if tentative_g_score >= self.distance_to_start(&neighbor) {
                    continue;
                }
                self.open.insert(neighbor);
            }
        }
        None
    }
    // pub fn solve_joint(mut self) -> Option<Vec<AxialPoint>> {
    //     let joints = self.solve_path()?;
    //     let mut path = vec![self.start];
    //     for joint in joints {
    //         path.push(joint.target());
    //     }
    //     Some(path)
    // }
    fn reconstruct_path(&self, current: &Point) -> Vec<Point> {
        let _ = vec![*current];
        todo!()
    }
}
