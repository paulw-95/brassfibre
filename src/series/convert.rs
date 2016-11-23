use std::convert::{From, Into};
use std::hash::Hash;

use super::Series;

impl<T: Copy> From<Vec<T>> for Series<T, usize> {
    fn from(values: Vec<T>) -> Self {
        Series::<T, usize>::from_vec(values)
    }
}

impl<T: Copy, U: Hash> Into<Vec<T>> for Series<T, U> {
    fn into(self) -> Vec<T> {
        self.values
    }
}

#[cfg(test)]
mod tests {

    use super::super::Series;

    #[test]
    fn test_from_vec_int() {
        let s: Series<i64, usize> = Series::from(vec![1, 2, 3]);
        let exp: Series<i64, usize> = Series::new(vec![1, 2, 3], vec![0, 1, 2]);
        assert_eq!(s, exp);
    }

    #[test]
    fn test_into_vec_int() {
        let s = Series::<i64, &str>::new(vec![1, 2, 3], vec!["a", "b", "c"]);
        let conv: Vec<i64> = s.into();
        assert_eq!(conv, vec![1, 2, 3]);
    }

    #[test]
    fn test_from_vec_str() {
        let s: Series<&str, usize> = Series::from(vec!["a", "b", "c"]);
        let exp: Series<&str, usize> = Series::new(vec!["a", "b", "c"], vec![0, 1, 2]);
        assert_eq!(s, exp);
    }

    #[test]
    fn test_into_vec_str() {
        let s = Series::<&str, &str>::new(vec!["a", "b", "c"], vec!["x", "y", "z"]);
        let conv: Vec<&str> = s.into();
        assert_eq!(conv, vec!["a", "b", "c"]);
    }
}