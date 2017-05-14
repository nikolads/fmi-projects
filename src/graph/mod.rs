use std::iter::DoubleEndedIterator;

mod adj_lists;

pub use self::adj_lists::AdjLists;

pub trait Graph {
    fn vertices<'a>(&'a self) -> Box<DoubleEndedIterator<Item=usize> + 'a>;

    fn neighbours<'a>(&'a self, node: usize) -> Box<DoubleEndedIterator<Item=usize> + 'a>;

    fn num_vertices(&self) -> usize {
        self.vertices().count()
    }

    fn num_edges(&self) -> usize {
        self.vertices().flat_map(|v| self.neighbours(v)).count()
    }
}
