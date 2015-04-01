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
        let next = self.cartesian.next()?;
        let point = self.map.relative_to_absolute(next.0, next.1);
        Some((point.x, point.y, &self.map.dense[(next.0, next.1)]))
    }
}

impl <'i, T> Iterator for MutGetTaxicabPoints<'i, T> {
    type Item = (isize, isize, &'i mut T);
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.cartesian.next()?;
        let point = self.map.relative_to_absolute(next.0, next.1);
        Some((point.x, point.y, &mut self.map.dense[(next.0, next.1)]))
    }
}

impl<'i, T> IntoIterator for &'i TaxicabMap<T> {
    type Item = (Point, &'i T);
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
    pub fn get_points(&self) -> GetTaxicabPoints<T> {
        let (w, h) = self.get_size();
        GetTaxicabPoints {
            map: self,
            cartesian: (0..w).cartesian_product(0..h),
        }
    }
    pub fn mut_points(&mut self) -> MutGetTaxicabPoints<T> {
        let (w, h) = self.get_size();
        MutGetTaxicabPoints {
            map: self,
            cartesian: (0..w).cartesian_product(0..h),
        }
    }
    pub fn rows(&self) -> impl Iterator<Item=impl Iterator<Item=(Point, &T)>> {
        let (w, h) = self.get_size();
        (0..h).map(move |y| {
            (0..w).map(move |x| {
                let point = self.relative_to_absolute(x, y);
                (point, &self.dense[(x, y)])
            })
        })
    }
    pub fn columns(&self) -> impl Iterator<Item=impl Iterator<Item=(Point, &T)>> {
        let (w, h) = self.get_size();
        (0..w).map(move |x| {
            (0..h).map(move |y| {
                let point = self.relative_to_absolute(x, y);
                (point, &self.dense[(x, y)])
            })
        })
    }
}