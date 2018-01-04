use std::hash::Hash;

use algos::grouper::{Grouper, HashGrouper};
use traits::RowIndex;

pub struct GroupBy<'a, D: 'a, G: Hash> {
    /// Grouped Series
    /// D: grouped data
    /// V: type of Group indexer
    pub data: &'a D,
    pub grouper: HashGrouper<G>,
}

impl<'a, D, G> GroupBy<'a, D, G>
where
    D: RowIndex<'a>,
    G: Clone + Eq + Hash + Ord,
{
    pub fn new(data: &'a D, indexer: &[G]) -> Self {

        assert!(
            data.len() == indexer.len(),
            "Series and Indexer length are different"
        );

        let grouper: HashGrouper<G> = HashGrouper::groupby(indexer);

        GroupBy {
            data: data,
            grouper: grouper,
        }
    }

    pub fn get_group(&self, group: &G) -> D {
        if let Some(locs) = self.grouper.get(group) {
            self.data.ilocs(locs)
        } else {
            panic!("Group not found!");
        }
    }

    pub fn groups(&self) -> Vec<G> {
        let mut keys: Vec<G> = self.grouper.keys();
        keys.sort();
        keys
    }
}
