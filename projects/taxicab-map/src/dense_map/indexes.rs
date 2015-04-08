use super::*;

impl<T> Index<(isize, isize)> for TaxicabMap<T> {
    type Output = T;

    fn index(&self, absolute: (isize, isize)) -> &Self::Output {
        let (x, y) = absolute;
        if cfg!(debug_assertions) {
            match self.get_point(x, y) {
                Some(s) => s,
                None => panic!("Index out of bounds: {:?}", absolute),
            }
        }
        else {
            unsafe { self.get_point(x, y).unwrap_unchecked() }
        }
    }
}

impl<T> IndexMut<(isize, isize)> for TaxicabMap<T> {
    fn index_mut(&mut self, absolute: (isize, isize)) -> &mut Self::Output {
        let (x, y) = absolute;
        if cfg!(debug_assertions) {
            match self.mut_point(x, y) {
                Some(s) => s,
                None => panic!("Index out of bounds: {:?}", absolute),
            }
        }
        else {
            unsafe { self.mut_point(x, y).unwrap_unchecked() }
        }
    }
}
