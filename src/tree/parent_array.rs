use super::Tree;

// Not exactly..
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

    fn add(&mut self, vert: usize, parent: usize) {
        self.data.push((parent, vert));
    }

    fn root(&self) -> usize {
        self.root
    }

    fn iter<'a>(&'a self) -> Box<Iterator<Item = (usize, usize)> + 'a> {
        Box::new(self.data.iter().cloned())
    }
}
