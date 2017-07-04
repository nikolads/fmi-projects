mod parent_array;

pub use self::parent_array::ParentArray;

pub trait Tree {
    fn new(root: usize) -> Self;

    fn add(&mut self, vert: usize, parent: usize);

    fn root(&self) -> usize;

    fn iter<'a>(&'a self) -> Box<Iterator<Item = (usize, usize)> + 'a>;
}
