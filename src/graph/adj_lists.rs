extern crate rand;

use self::rand::{StdRng, Rng, SeedableRng};

use std::iter::IntoIterator;
use std::collections::HashSet;
use std::ops::Range;

use super::Graph;

#[derive(Debug, Clone)]
pub struct AdjLists {
    lists: Vec<Vec<u32>>,
}

impl AdjLists {
    pub fn new(verts: u32) -> Self {
        AdjLists {
            lists: vec![Vec::new(); verts as usize],
        }
    }

    pub fn from_parts<I>(verts: u32, iter: I) -> Self where
        I: IntoIterator<Item = Part>
    {
        let mut graph = Self::new(verts);

        for part in iter {
            debug_assert!(part.owned_verts.end <= verts);
            debug_assert!(part.target_verts.end <= verts);

            // cannot use `part.index_to_vert()` because of borrowing rules
            let start_index = part.owned_verts.start as usize;

            for (v, list) in part.lists.into_iter().enumerate().map(|(i, list)| (i + start_index, list)) {
                debug_assert!(graph.lists[v].is_empty());
                graph.lists[v] = list;
            }
        }

        graph
    }
}

impl Graph for AdjLists {
    fn vertices(&self) -> Box<Iterator<Item=u32>> {
        Box::new((0 as u32)..(self.lists.len() as u32))
    }

    fn neighbours<'a>(&'a self, vert: u32) -> Box<Iterator<Item=u32> + 'a> {
        Box::new(self.lists[vert as usize].iter().cloned())
    }
}

#[derive(Debug, Clone)]
pub struct Part {
    owned_verts: Range<u32>,
    target_verts: Range<u32>,
    lists: Vec<Vec<u32>>
}

impl Part {
    pub fn new(owned: Range<u32>, target: Range<u32>) -> Self {
        assert!(owned.start <= owned.end);
        assert!(target.start <= target.end);

        let len = (owned.end - owned.start) as usize;

        Part {
            owned_verts: owned,
            target_verts: target,
            lists: vec![Vec::new(); len],
        }
    }

    pub fn add_edge(&mut self, from: u32, to: u32) {
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

        let mut edges = HashSet::with_capacity(n_edges as usize);

        while edges.len() != n_edges as usize {
            let from = rng.gen::<u32>() % n_owned;
            let from = self.index_to_vert(from as usize);
            let to = rng.gen::<u32>() % n_target;

            edges.insert((from, to));
        }

        for (u, v) in edges {
            self.add_edge(u, v);
        }

        for list in &mut self.lists {
            list.sort();
        }
    }

    #[inline]
    fn vert_to_index(&self, n: u32) -> usize {
        (n - self.owned_verts.start) as usize
    }

    #[inline]
    fn index_to_vert(&self, n: usize) -> u32 {
        n as u32 + self.owned_verts.start
    }
}
