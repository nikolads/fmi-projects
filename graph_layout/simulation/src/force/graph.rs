use std::iter::{self, FlatMap, Enumerate, Repeat, Zip};
use std::ops::Range;
use std::slice::Iter;

pub struct Graph {
    vertices: Vec<()>,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self) {
        self.vertices.push(());
        self.edges.push(Vec::new());
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        assert!(from < self.vertices.len() && to < self.vertices.len());

        self.edges[from].push(to);
        self.edges[to].push(from);
    }

    pub fn vertices(&self) -> Range<usize> {
        0..self.vertices.len()
    }

    pub fn edges(&self) -> EdgesIter {
        fn helper((v, list): (usize, &Vec<usize>)) -> Zip<Iter<usize>, Repeat<usize>> {
            list.iter().zip(iter::repeat(v))
        }

        self.edges.iter().enumerate().flat_map(helper)
    }
}

pub type EdgesIter<'a> = FlatMap<
    Enumerate<Iter<'a, Vec<usize>>>,
    Zip<Iter<'a, usize>, Repeat<usize>>,
    fn((usize, &'a Vec<usize>)) -> Zip<Iter<'a, usize>, Repeat<usize>>>;
