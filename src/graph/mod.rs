mod adj_lists;

pub use self::adj_lists::AdjLists;

pub trait GraphPart {
    fn owned_vertices<'a>(&'a self) -> Box<Iterator<Item=usize> + 'a>;
    fn target_vertices<'a>(&'a self) -> Box<Iterator<Item=usize> + 'a>;
    fn neighbours<'a>(&'a self, vert: usize) -> Box<Iterator<Item=usize> + 'a>;

    fn num_owned_vertices(&self) -> usize {
        self.owned_vertices().count()
    }

    fn num_target_vertices(&self) -> usize {
        self.target_vertices().count()
    }

    fn num_edges(&self) -> usize {
        self.owned_vertices().flat_map(|v| self.neighbours(v)).count()
    }
}
