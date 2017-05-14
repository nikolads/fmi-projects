use super::Tree;

#[derive(Debug)]
pub struct ParentArray {
    root: usize,
    data: Vec<(usize, usize)>,
}

impl Tree for ParentArray {
    fn new(root: usize) -> ParentArray {
        ParentArray {
            root: root,
            data: Vec::new(),
        }
    }

    fn add(&mut self, parent: usize, vert: usize) {
        self.data.push((parent, vert));
    }
}
