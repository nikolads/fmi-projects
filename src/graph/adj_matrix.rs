extern crate rand;

use self::rand::{StdRng, SeedableRng};
use self::rand::distributions::Sample;
use self::rand::distributions::range::Range as RngRange;

use std::cell::UnsafeCell;
use std::ops::Range;
use std::sync::Arc;

use super::Graph;

#[derive(Debug)]
pub struct Matrix {
    data: Vec<bool>,
    // TODO
    borrowed_flag: bool,
}

// TODO: use bitmap
#[derive(Debug)]
pub struct AdjMatrix {
    n_vert: usize,
    matrix: Arc<UnsafeCell<Matrix>>,
}

impl AdjMatrix {
    pub fn new(n_vert: usize) -> Self {
        AdjMatrix {
            n_vert: n_vert,
            matrix: Arc::new(UnsafeCell::new(Matrix { data: vec![false; n_vert * n_vert], borrowed_flag: false })),
        }
    }

    pub fn parts<'a>(&'a self, count: usize) -> impl Iterator<Item = Part> + 'a {
        unsafe {
            assert!(!(*self.matrix.get()).borrowed_flag);
            (*self.matrix.get()).borrowed_flag = true;
        }

        (0..count+1).map(move |k| {
            let prev = (k as f64 / (count + 2) as f64).sqrt();
            let split = ((k + 1) as f64 / (count + 2) as f64).sqrt();

            let beg = (self.n_vert as f64 * prev) as usize;
            let end = (self.n_vert as f64 * split) as usize;

            Part::new(beg..end, self.n_vert, &self.matrix)
        })
    }
}

impl Graph for AdjMatrix {
    fn vertices(&self) -> Box<Iterator<Item = usize>> {
        Box::new((0..self.n_vert).into_iter())
    }

    fn neighbours<'a>(&'a self, vert: usize) -> Box<Iterator<Item = usize> + 'a> {
        let slice = unsafe { &(*self.matrix.get()).data[(vert * self.n_vert)..((vert+1) * self.n_vert)] };
        Box::new(slice.iter().enumerate().filter_map(|(i, &edge)| {
            if edge {
                Some(i)
            } else {
                None
            }
        }))
    }
}

// what could possibly go wrong
unsafe impl Send for Part{}

#[derive(Debug, Clone)]
pub struct Part {
    owned_verts: Range<usize>,
    total: usize,
    matrix: Arc<UnsafeCell<Matrix>>
}

impl Part {
    pub fn new(owned: Range<usize>, total: usize, matrix: &Arc<UnsafeCell<Matrix>>) -> Self {
        assert!(owned.start <= owned.end);

        Part {
            owned_verts: owned.clone(),
            total: total,
            matrix: matrix.clone(),
        }
    }

    // pub fn add_edge(&mut self, from: usize, to: usize) {
    //     assert!(self.owned_verts.contains(from));
    //     assert!(self.target_verts.contains(to));

    //     let indx = self.vert_to_index(from);
    //     self.lists[indx].push(to);
    // }

    pub fn generate_edges_undirected(&mut self, n_edges: usize, seed: Option<usize>) {
        let mut rng = match seed {
            Some(seed) => StdRng::from_seed(&[seed]),
            None => StdRng::new().unwrap(),
        };

        println!("{:?}", self.owned_verts);
        println!("{:?}", n_edges);

        let mut owned = RngRange::new(self.owned_verts.start, self.owned_verts.end);
        let mut cnt = 0;

        while cnt < n_edges {
            let from = owned.sample(&mut rng);

            let mut target = RngRange::new(0, self.total - from);
            let to = target.sample(&mut rng);

            unsafe {
                if from != to && !(*self.matrix.get()).data[from * self.total + to] {
                    (*self.matrix.get()).data[from * self.total + to] = true;
                    (*self.matrix.get()).data[to * self.total + from] = true;
                    cnt += 1;
                }
            }
        }
    }
}

