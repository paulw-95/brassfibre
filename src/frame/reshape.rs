use std::hash::Hash;

use super::DataFrame;
use super::super::algos::join::{Join, HashJoin};
use super::super::internals::Array;
use super::super::traits::{IndexerIndexer, RowIndexer,
                           Appender, Concatenator, Joiner};


impl<U, V> Appender for DataFrame<U, V>
    where U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    fn append(&self, other: &Self) -> Self {
        assert!(self.columns == other.columns, "columns must be identical");

        let new_index = self.index.append(&other.index);

        let mut new_values: Vec<Array> = Vec::with_capacity(self.columns.len());
        for (svalues, ovalues) in self.values.iter().zip(&other.values) {
            let new_value = svalues.append(&ovalues);
            new_values.push(new_value);
        }

        DataFrame::from_vec(new_values, new_index, self.columns.clone())
    }
}

impl<U, V> Concatenator for DataFrame<U, V>
    where U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    fn concat(&self, other: &Self) -> Self {
        assert!(self.index == other.index, "index must be identical");

        let new_columns = self.columns.append(&other.columns);

        let mut new_values: Vec<Array> = Vec::with_capacity(new_columns.len());
        for values in self.values.iter().chain(&other.values) {
            new_values.push(values.clone());
        }

        DataFrame::from_vec(new_values, self.index.clone(), new_columns)
    }
}

impl<U, V> Joiner for DataFrame<U, V>
    where U: Copy + Eq + Hash,
          V: Copy + Eq + Hash {

    fn join_inner(&self, other: &Self) -> Self {

        let (new_index, lindexer, rindexer) = HashJoin::inner(&self.index.values, &other.index.values);
        let new_columns = self.columns.append(&other.columns);

        let mut new_values: Vec<Array> = Vec::with_capacity(new_columns.len());

        for values in self.ilocs(&lindexer).values {
            new_values.push(values.clone());
        }
        for values in other.ilocs(&rindexer).values {
            new_values.push(values.clone());
        }

        DataFrame::from_vec(new_values, new_index, new_columns)
    }
}
