use std::ops::Range;
use itertools::Product;
use super::*;


pub struct GetTaxicabPoints<'i, T> {
    map: &'i TaxicabMap<T>,
    cartesian: Product<Range<usize>, Range<usize>>,
}

pub struct MutGetTaxicabPoints<'i, T> {
    map: &'i mut TaxicabMap<T>,
    cartesian: Product<Range<usize>, Range<usize>>,
}


impl<'i, T> Iterator for GetTaxicabPoints<'i, T> {
    type Item = (isize, isize, &'i T);
    fn next(&mut self) -> Option<Self::Item> {
        let (i, j) = self.cartesian.next()?;
        let (x, y) = relative_to_absolute(i, j);
        let v = self.map.get_point(x, y)?;
        Some((x, y, v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.cartesian.size_hint()
    }
}

// impl<'i, 'a,  T> Iterator for MutGetTaxicabPoints<'i, T> {
//     type Item = (isize, isize, &'i mut T);
//     fn next(&'a mut self) -> Option<Self::Item> {
//         let (i, j) = self.cartesian.next()?;
//         let (x, y) = self.map.relative_to_absolute(i, j);
//         let v = self.map.mut_point(x, y)?;
//         Some((x, y, v))
//     }
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         self.cartesian.size_hint()
//     }
// }

impl<'i, T> IntoIterator for &'i TaxicabMap<T> {
    type Item = (isize, isize, &'i T);
    type IntoIter = GetTaxicabPoints<'i, T>;
    fn into_iter(self) -> Self::IntoIter {
        let (w, h) = self.get_size();
        GetTaxicabPoints {
            map: self,
            cartesian: (0..w).cartesian_product(0..h),
        }
    }
}


impl<T> TaxicabMap<T> {
    pub fn points_all(&self) -> GetTaxicabPoints<T> {
        let (w, h) = self.get_size();
        GetTaxicabPoints {
            map: self,
            cartesian: (0..w).cartesian_product(0..h),
        }
    }
    pub fn points_mut(&mut self) -> MutGetTaxicabPoints<T> {
        let (w, h) = self.get_size();
        MutGetTaxicabPoints {
            map: self,
            cartesian: (0..w).cartesian_product(0..h),
        }
    }
}


impl<T> TaxicabMap<T> {
    /// Find at most 4 points that are exists and adjacent to a direction.
    pub fn points_nearby(&self, x: isize, y: isize) -> impl Iterator<Item=(isize, isize)> {
        let (w, h) = self.get_size();
        let (i, j) = absolute_to_relative(x, y,  self.origin_x, self.origin_y, w, h, self.cycle_x, self.cycle_y);
        Direction::all()
            .into_iter()
            .map(move |v| v.as_joint(x, y).target())
            .filter(move |(x, y)| {
                i < w && j < h
            })
    }
    /// Find all points that are within a certain distance of a direction.
    pub fn points_around(&self, x: isize, y: isize, distance: usize) -> Vec<(isize, isize)> {
        match distance {
            0 => vec![(x, y)],
            1 => self.points_nearby(x, y).collect_vec(),
            // TODO: optimize this
            _ => {
                let mut points = vec![];
                for (x, y) in self.points_around(x, y, distance - 1) {
                    for (x, y) in self.points_nearby(x, y) {
                        if !points.contains(&(x, y)) {
                            points.push((x, y));
                        }
                    }
                }
                points
            }
        }
    }
}
