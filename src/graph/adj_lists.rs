extern crate rand;

use self::rand::{StdRng, Rng, SeedableRng};

use std::iter::IntoIterator;
use std::collections::HashSet;
use std::ops::Range;

use super::Graph;

#[derive(Debug, Clone)]
pub struct AdjLists {
    lists: Vec<Vec<usize>>,
}

impl AdjLists {
    pub fn new(verts: usize) -> Self {
        AdjLists {
            lists: vec![Vec::new(); verts],
        }
    }

    pub fn from_parts<I>(verts: usize, iter: I) -> Self where
        I: IntoIterator<Item = Part>
    {
        let mut graph = Self::new(verts);

        for part in iter {
            debug_assert!(part.owned_verts.end <= verts);
            debug_assert!(part.target_verts.end <= verts);

            // cannot use `part.index_to_vert()` because of borrowing rules
            let start_index = part.owned_verts.start;

            for (v, list) in part.lists.into_iter().enumerate().map(|(i, list)| (i + start_index, list)) {
                debug_assert!(graph.lists[v].is_empty());
                graph.lists[v] = list;
            }
        }

        graph
    }
}

impl Graph for AdjLists {
    fn vertices(&self) -> Box<Iterator<Item=usize>> {
        Box::new((0)..(self.lists.len()))
    }

    fn neighbours<'a>(&'a self, vert: usize) -> Box<Iterator<Item=usize> + 'a> {
        Box::new(self.lists[vert].iter().cloned())
    }
}

#[derive(Debug, Clone)]
pub struct Part {
    owned_verts: Range<usize>,
    target_verts: Range<usize>,
    lists: Vec<Vec<usize>>
}

impl Part {
    pub fn new(owned: Range<usize>, target: Range<usize>) -> Self {
        assert!(owned.start <= owned.end);
        assert!(target.start <= target.end);

        let len = owned.end - owned.start;

        Part {
            owned_verts: owned,
            target_verts: target,
            lists: vec![Vec::new(); len],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        assert!(self.owned_verts.contains(from));
        assert!(self.target_verts.contains(to));

        let indx = self.vert_to_index(from);
        self.lists[indx].push(to);
    }

    pub fn generate_edges_directed(&mut self, n_edges: usize, seed: Option<usize>) {
        let mut rng = match seed {
            Some(seed) => StdRng::from_seed(&[seed]),
            None => StdRng::new().unwrap(),
        };

        let n_owned = self.owned_verts.end - self.owned_verts.start;
        let n_target = self.target_verts.end - self.target_verts.start;

        let mut edges = HashSet::with_capacity(n_edges);

        while edges.len() != n_edges {
            let from = rng.gen::<usize>() % n_owned;
            let from = self.index_to_vert(from);
            let to = rng.gen::<usize>() % n_target;

            if from != to {
                edges.insert((from, to));
            }
        }

        for (u, v) in edges {
            self.add_edge(u, v);
        }

        for list in &mut self.lists {
            list.sort();
        }
    }

    #[inline]
    fn vert_to_index(&self, n: usize) -> usize {
        (n - self.owned_verts.start)
    }

    #[inline]
    fn index_to_vert(&self, n: usize) -> usize {
        n + self.owned_verts.start
    }
}
