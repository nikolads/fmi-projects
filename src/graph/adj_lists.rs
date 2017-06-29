extern crate rand;

use super::GraphPart;
use self::rand::{StdRng, Rng, SeedableRng};
use std::collections::HashSet;
use std::ops::Range;

#[derive(Debug)]
pub struct AdjLists {
    owned_verts: Range<usize>,
    all_verts: Range<usize>,
    lists: Vec<Vec<usize>>,
}

impl AdjLists {
    pub fn new(owned_verts: Range<usize>, all_verts: Range<usize>) -> AdjLists {
        assert!(owned_verts.start <= owned_verts.end);
        assert!(all_verts.start <= all_verts.end);

        AdjLists {
            owned_verts: owned_verts.clone(),
            all_verts: all_verts,
            lists: vec![Vec::new(); owned_verts.end - owned_verts.start],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        assert!(self.owned_verts.contains(from));
        assert!(self.all_verts.contains(to));

        let indx = self.vert_to_index(from);
        self.lists[indx].push(to);
    }

    pub fn generate_edges_directed(&mut self, n_edges: usize, seed: Option<usize>) {
        let mut edges = HashSet::with_capacity(n_edges as usize);

        let mut rng = match seed {
            Some(seed) => StdRng::from_seed(&[seed]),
            None => StdRng::new().unwrap(),
        };

        let n_owned = self.num_owned_vertices();
        let n_target = self.num_target_vertices();

        while edges.len() != n_edges as usize {
            let from = rng.gen::<usize>() % n_owned;
            let from = self.index_to_vert(from);
            let to = rng.gen::<usize>() % n_target;

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
    fn vert_to_index(&self, vert: usize) -> usize {
        vert - self.owned_verts.start
    }

    #[inline]
    fn index_to_vert(&self, index: usize) -> usize {
        index + self.owned_verts.start
    }
}

impl GraphPart for AdjLists {
    fn owned_vertices(&self) -> Box<Iterator<Item=usize>> {
        Box::new(self.owned_verts.clone())
    }

    fn target_vertices(&self) -> Box<Iterator<Item=usize>> {
        Box::new(self.all_verts.clone())
    }

    fn neighbours<'a>(&'a self, vert: usize) -> Box<Iterator<Item=usize> + 'a> {
        let indx = self.vert_to_index(vert);
        Box::new(self.lists[indx].iter().cloned())
    }
}
