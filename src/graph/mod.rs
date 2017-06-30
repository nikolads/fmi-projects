pub mod adj_lists;

// TODO: remove the box when either higher-kindered associated items or
// impl Trait in Traits are implemented
pub trait Graph {
    fn vertices<'a>(&'a self) -> Box<Iterator<Item = u32> + 'a>;
    fn neighbours<'a>(&'a self, vert: u32) -> Box<Iterator<Item = u32> + 'a>;

    fn num_vertices(&self) -> usize {
        self.vertices().count()
    }

    fn num_edges(&self) -> usize {
        self.vertices().flat_map(|v| self.neighbours(v)).count()
    }
}
