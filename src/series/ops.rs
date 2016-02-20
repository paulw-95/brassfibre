extern crate num;

use num::{Num};
use std::hash::Hash;
use std::ops::{Add, Mul, Sub, Div, Rem};

use super::Series;

macro_rules! define_numric_op(
  ($t:ident $m:ident) => (

    // Broadcast
    impl<T, U> $t<T> for Series<T, U>
        where T: Copy + Num,
              U: Copy + Eq + Hash {

        type Output = Series<T, U>;
        fn $m(self, _rhs: T) -> Series<T, U> {
            let new_values = self.values.iter().map(|x: &T| (*x).$m(_rhs)).collect();
            return Series::new(new_values, self.index.copy_values());
        }
    }

    // Element-wise
    impl<T, U> $t<Series<T, U>> for Series<T, U>
        where T: Copy + Num,
              U: Copy + Eq + Hash {

        type Output = Series<T, U>;
        fn $m(self, _rhs: Series<T, U>) -> Series<T, U> {
            if !self.index.equals(&_rhs.index) {
                panic!("index must be the same!");
            }
            let new_values = self.values.iter()
                                 .zip(_rhs.values.iter())
                                 .map(|(x, y)| (*x).$m(*y)).collect();
            return Series::new(new_values, self.index.copy_values());
        }
    }

  );
);

define_numric_op!(Add add);
define_numric_op!(Mul mul);
define_numric_op!(Sub sub);
define_numric_op!(Div div);
define_numric_op!(Rem rem);

#[cfg(test)]
mod tests {

    use super::super::Series;

    #[test]
    fn test_series_ops_i64_broadcast() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        // s moves by ops
        let result = s + 3;
        assert_eq!(&result.values, &vec![4, 5, 6]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let result = s * 2;
        assert_eq!(&result.values, &vec![2, 4, 6]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let result = s - 3;
        assert_eq!(&result.values, &vec![-2, -1, 0]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let result = s / 2;
        assert_eq!(&result.values, &vec![0, 1, 1]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let result = s % 2;
        assert_eq!(&result.values, &vec![1, 0, 1]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_f64_broadcast() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        // s moves by ops
        let result = s + 3.;
        assert_eq!(&result.values, &vec![4., 5., 6.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let result = s * 2.;
        assert_eq!(&result.values, &vec![2., 4., 6.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let result = s - 3.;
        assert_eq!(&result.values, &vec![-2., -1., 0.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let result = s / 2.;
        assert_eq!(&result.values, &vec![0.5, 1., 1.5]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let result = s % 2.;
        assert_eq!(&result.values, &vec![1., 0., 1.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_i64_elemwise() {
        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        // s moves by ops
        let result = s + r;
        assert_eq!(&result.values, &vec![2, 5, 5]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = s * r;
        assert_eq!(&result.values, &vec![1, 6, 6]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = s - r;
        assert_eq!(&result.values, &vec![0, -1, 1]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = s / r;
        assert_eq!(&result.values, &vec![1, 0, 1]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<i64, i64>::new(vec![1, 2, 3], vec![10, 20, 30]);
        let r = Series::<i64, i64>::new(vec![1, 3, 2], vec![10, 20, 30]);
        let result = s % r;
        assert_eq!(&result.values, &vec![0, 2, 1]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
    }

    #[test]
    fn test_series_ops_f64_elemwise() {
        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        // s moves by ops
        let result = s + r;
        assert_eq!(&result.values, &vec![2., 5., 5.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = s * r;
        assert_eq!(&result.values, &vec![1., 6., 6.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = s - r;
        assert_eq!(&result.values, &vec![0., -1., 1.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = s / r;
        assert_eq!(&result.values, &vec![1., 0.6666666666666666, 1.5]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);

        let s = Series::<f64, i64>::new(vec![1., 2., 3.], vec![10, 20, 30]);
        let r = Series::<f64, i64>::new(vec![1., 3., 2.], vec![10, 20, 30]);
        let result = s % r;
        assert_eq!(&result.values, &vec![0., 2., 1.]);
        assert_eq!(&result.index.values, &vec![10, 20, 30]);
    }
}
