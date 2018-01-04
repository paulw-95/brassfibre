use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::slice;
use std::vec;

use nullvec::prelude::Array;

use groupby::GroupBy;
use indexer::Indexer;
use traits::{Slicer, IndexerIndex, RowIndex, ColIndex};

mod aggregation;
mod formatting;
mod reshape;

#[derive(Clone)]
pub struct DataFrame<'v, 'i, 'c, I, C>
where
    I: 'i + Clone + Hash,
    C: 'c + Clone + Hash,
{
    /// 2-dimentional block contains multiple type.
    /// I: type of indexer
    /// C: type of columns
    pub values: Vec<Cow<'v, Array>>,
    pub index: Cow<'i, Indexer<I>>,
    pub columns: Cow<'c, Indexer<C>>,
}

/// /////////////////////////////////////////////////////////////////////////////
/// Indexing
/// /////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, 'c, I, C> RowIndex<'c> for DataFrame<'v, 'i, 'c, I, C>
where
    I: Clone + Eq + Hash,
    C: Clone + Eq + Hash,
{
    type Key = I;
    type Row = Array;

    fn len(&'c self) -> usize {
        self.index.len()
    }

    fn loc(&'c self, _label: &Self::Key) -> Self::Row {
        unimplemented!()
    }

    fn iloc(&'c self, _locaiton: &usize) -> Self::Row {
        unimplemented!()
    }

    fn reindex<'l>(&'c self, labels: &'l [Self::Key]) -> Self {
        let locations = self.index.get_locs(labels);
        self.reindex_by_index(&locations)
    }

    fn reindex_by_index<'l>(&'c self, locations: &'l [usize]) -> Self {
        let new_index = self.index.reindex(locations);
        // boudaries are checked in Indexer.reindex

        let mut new_values: Vec<Cow<Array>> = Vec::with_capacity(self.columns.len());
        for current in &self.values {
            let new_value = unsafe { current.ilocs_unchecked(locations) };
            new_values.push(Cow::Owned(new_value));
        }
        DataFrame::from_cow(
            new_values,
            Cow::Owned(new_index),
            Cow::Borrowed(self.columns.borrow()),
        )
    }

    fn blocs(&self, _labels: &[bool]) -> Self {
        unimplemented!()
        // ToDo: fix Series impl
    }
}

impl<'v, 'i, 'c, I, C> ColIndex<'i> for DataFrame<'v, 'i, 'c, I, C>
where
    I: Clone + Eq + Hash,
    C: Clone + Eq + Hash,
{
    type Key = C;
    type Column = Array;

    fn get(&'i self, _label: &Self::Key) -> Self::Column {
        unimplemented!();
    }

    fn iget(&'i self, _loc: &usize) -> Self::Column {
        unimplemented!();
    }

    fn gets<'l>(&'i self, labels: &'l [Self::Key]) -> Self {
        let locs = self.columns.get_locs(labels);
        self.igets(&locs)
    }

    fn igets<'l>(&'i self, locations: &'l [usize]) -> Self {
        let new_columns = self.columns.reindex(locations);

        let mut new_values: Vec<Cow<Array>> = Vec::with_capacity(new_columns.len());
        for loc in locations {
            // new_values.push(Cow::Borrowed(self.values[*loc].borrow()));
            new_values.push(Cow::Owned(self.values[*loc].clone().into_owned()));
        }
        DataFrame::from_cow(
            new_values,
            Cow::Borrowed(self.index.borrow()),
            Cow::Owned(new_columns),
        )
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Misc
/// /////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, 'c, I, C> DataFrame<'v, 'i, 'c, I, C>
where
    I: Clone + Eq + Hash,
    C: Clone + Eq + Hash,
{
    pub fn from_vec<X, Y>(values: Vec<Array>, index: X, columns: Y) -> Self
    where
        X: Into<Indexer<I>>,
        Y: Into<Indexer<C>>,
    {

        let index: Indexer<I> = index.into();
        let columns: Indexer<C> = columns.into();

        assert!(values.len() == columns.len(), "Length mismatch!");
        let values: Vec<Cow<Array>> = values.into_iter().map(Cow::Owned).collect();

        let len = index.len();
        for value in &values {
            assert!(value.len() == len, "Length mismatch!");
        }
        DataFrame {
            values: values,
            index: Cow::Owned(index),
            columns: Cow::Owned(columns),
        }
    }

    fn from_cow(
        values: Vec<Cow<'v, Array>>,
        index: Cow<'i, Indexer<I>>,
        columns: Cow<'c, Indexer<C>>,
    ) -> Self {
        // temp internal, use IntoCow
        DataFrame {
            values: values,
            index: index,
            columns: columns,
        }
    }

    pub fn dtypes(&self) -> Vec<String> {
        self.iter().map(|x| x.dtype()).collect()
    }

    pub fn is_numeric(&self) -> Vec<bool> {
        self.iter().map(|x| x.is_numeric()).collect()
    }

    fn get_numeric_data(&'i self) -> DataFrame<'i, 'i, 'i, I, C> {
        let flags = self.is_numeric();
        // ToDo: use bgets
        let indexer: Vec<usize> = flags
            .iter()
            .enumerate()
            .filter(|&(_, &f)| f)
            .map(|(i, _)| i)
            .collect();
        self.igets(&indexer)
    }

    pub fn insert(&mut self, values: Array, name: C) {
        assert!(self.len() == values.len(), "Length mismatch!");

        self.values.push(Cow::Owned(values));
        self.columns.to_mut().push(name);
    }

    pub fn groupby<G>(&'i self, other: &[G]) -> GroupBy<DataFrame<I, C>, G>
    where
        G: Clone + Eq + Hash + Ord,
    {

        GroupBy::new(self, other)
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Eq
/// /////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, 'c, I, C> PartialEq for DataFrame<'v, 'i, 'c, I, C>
where
    I: Clone + Hash + Eq,
    C: Clone + Hash + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        (self.index.eq(&other.index)) && (self.columns.eq(&other.columns)) &&
            (self.values.eq(&other.values))
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Iterator
/// /////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, 'c, I, C> IntoIterator for DataFrame<'v, 'i, 'c, I, C>
where
    I: Clone + Hash + Eq,
    C: Clone + Hash + Eq,
{
    type Item = Cow<'v, Array>;
    type IntoIter = vec::IntoIter<Cow<'v, Array>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'v, 'i, 'c, I, C> DataFrame<'v, 'i, 'c, I, C>
where
    I: Clone + Hash + Eq,
    C: Clone + Hash + Eq,
{
    pub fn iter(&self) -> slice::Iter<Cow<Array>> {
        self.values.iter()
    }
}

#[cfg(test)]
mod tests {

    use nullvec::prelude::Array;

    use super::DataFrame;
    use traits::{RowIndex, ColIndex};

    #[test]
    fn test_block_creation_from_vec() {
        let values = vec![
            Array::new(vec![1, 2, 3, 4, 5]),
            Array::new(vec![6., 7., 8., 9., 10.]),
            Array::new(vec![11, 12, 13, 14, 15]),
        ];
        let df = DataFrame::from_vec(
            values,
            vec!["A", "BB", "CC", "D", "EEE"],
            vec!["X", "YYY", "ZZ"],
        );
        assert_eq!(df.len(), 5);
    }

    #[test]
    fn test_block_add_columns() {
        let values = vec![Array::new(vec![1, 2, 3]), Array::new(vec![4., 5., 6.])];
        let mut df = DataFrame::from_vec(values, vec!["A", "BB", "CC"], vec!["X", "Y"]);
        assert_eq!(df.len(), 3);
        df.insert(Array::new(vec![10, 11, 12]), "Z");

        let exp_values = vec![
            Array::new(vec![1, 2, 3]),
            Array::new(vec![4., 5., 6.]),
            Array::new(vec![10, 11, 12]),
        ];
        let exp = DataFrame::from_vec(exp_values, vec!["A", "BB", "CC"], vec!["X", "Y", "Z"]);
        assert_eq!(df.values, exp.values);
        assert_eq!(df.index, exp.index);
        assert_eq!(df.columns, exp.columns);
    }

    #[test]
    fn test_block_slice_locs() {
        let values = vec![
            Array::new(vec![1, 2, 3, 4, 5]),
            Array::new(vec![6., 7., 8., 9., 10.]),
            Array::new(vec![11, 12, 13, 14, 15]),
        ];
        let df = DataFrame::from_vec(
            values,
            vec!["A", "BB", "CC", "D", "EEE"],
            vec!["X", "YYY", "ZZ"],
        );
        assert_eq!(df.len(), 5);

        let res = df.locs(&vec!["A", "D", "CC"]);
        let exp_values = vec![
            Array::new(vec![1, 4, 3]),
            Array::new(vec![6., 9., 8.]),
            Array::new(vec![11, 14, 13]),
        ];
        let exp = DataFrame::from_vec(exp_values, vec!["A", "D", "CC"], vec!["X", "YYY", "ZZ"]);
        assert_eq!(res.values, exp.values);
        assert_eq!(res.index, exp.index);
        assert_eq!(res.columns, exp.columns);
    }

    #[test]
    fn test_block_slice_ilocs() {
        let values = vec![
            Array::new(vec![1, 2, 3, 4, 5]),
            Array::new(vec![6., 7., 8., 9., 10.]),
            Array::new(vec![11, 12, 13, 14, 15]),
        ];
        let df = DataFrame::from_vec(
            values,
            vec!["A", "BB", "CC", "D", "EEE"],
            vec!["X", "YYY", "ZZ"],
        );
        assert_eq!(df.len(), 5);

        let res = df.ilocs(&vec![0, 3, 2]);
        let exp_values = vec![
            Array::new(vec![1, 4, 3]),
            Array::new(vec![6., 9., 8.]),
            Array::new(vec![11, 14, 13]),
        ];
        let exp = DataFrame::from_vec(exp_values, vec!["A", "D", "CC"], vec!["X", "YYY", "ZZ"]);
        assert_eq!(res.values, exp.values);
        assert_eq!(res.index, exp.index);
        assert_eq!(res.columns, exp.columns);
    }

    #[test]
    fn test_block_columns_slice() {
        let values = vec![
            Array::new(vec![1, 2, 3, 4, 5]),
            Array::new(vec![6., 7., 8., 9., 10.]),
            Array::new(vec![11, 12, 13, 14, 15]),
        ];
        let b = DataFrame::from_vec(
            values,
            vec!["A", "BB", "CC", "D", "EEE"],
            vec!["X", "YYY", "ZZ"],
        );

        let exp_values = vec![
            Array::new(vec![6., 7., 8., 9., 10.]),
            Array::new(vec![1, 2, 3, 4, 5]),
        ];
        let exp = DataFrame::from_vec(
            exp_values,
            vec!["A", "BB", "CC", "D", "EEE"],
            vec!["YYY", "X"],
        );
        let res = b.gets(&vec!["YYY", "X"]);
        assert_eq!(res.values, exp.values);
        assert_eq!(res.index, exp.index);
        assert_eq!(res.columns, exp.columns);

        let res = b.igets(&vec![1, 0]);
        assert_eq!(res.values, exp.values);
        assert_eq!(res.index, exp.index);
        assert_eq!(res.columns, exp.columns);
    }

}
