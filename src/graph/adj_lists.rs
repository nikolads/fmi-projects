use super::Graph;

use std::cmp;
use std::iter::DoubleEndedIterator;

pub struct AdjLists {
    lists: Vec<Vec<usize>>
}

impl AdjLists {
    pub fn new() -> AdjLists {
        AdjLists {
            lists: Vec::new()
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        let max = cmp::max(from, to) + 1;

        if self.lists.len() < max {
            self.lists.resize(max, Vec::new());
        }

        self.lists[from].push(to);
    }
}

impl Graph for AdjLists {
    fn vertices(&self) -> Box<DoubleEndedIterator<Item=usize>> {
        Box::new(0 .. self.lists.len())
    }

    fn neighbours<'a>(&'a self, node:usize) -> Box<DoubleEndedIterator<Item=usize> + 'a> {
        Box::new(self.lists[node].iter().cloned())
    }

    fn num_vertices(&self) -> usize {
        self.lists.len()
    }
}
