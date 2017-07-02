pub mod adj_lists;
pub mod adj_matrix;

// TODO: remove the box when either higher-kindered associated items or
// impl Trait in Traits are implemented
pub trait Graph {
    fn vertices<'a>(&'a self) -> Box<Iterator<Item = usize> + 'a>;
    fn neighbours<'a>(&'a self, vert: usize) -> Box<Iterator<Item = usize> + 'a>;

    fn num_vertices(&self) -> usize {
        self.vertices().count()
    }

    fn num_edges(&self) -> usize {
        self.vertices().flat_map(|v| self.neighbours(v)).count()
    }
}
