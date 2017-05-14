mod parent_array;

pub use self::parent_array::ParentArray;

pub trait Tree {
    fn new(root: usize) -> Self;

    fn add(&mut self, parent: usize, vert: usize);
}
