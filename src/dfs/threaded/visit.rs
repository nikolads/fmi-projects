use std::cmp;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::usize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Parent {
    pub root: usize,
    pub branch: usize,
    pub vertex: usize,
}

impl Parent {
    pub fn new(root: usize, vertex: usize, branch: usize) -> Self {
        Parent {
            root: root,
            vertex: vertex,
            branch: branch,
        }
    }
}

pub struct Visited {
    root: AtomicUsize,
    branch: AtomicUsize,
    vertex: AtomicUsize,
    mutex: Mutex<()>,
}

impl Visited {
    pub fn from_parent(parent: &Parent) -> Self {
        Visited {
            root: AtomicUsize::new(parent.root),
            branch: AtomicUsize::new(parent.branch),
            vertex: AtomicUsize::new(parent.vertex),
            mutex: Mutex::new(()),
        }
    }

    /// A value that is bigger than all else.
    pub fn max() -> Self {
        Visited {
            root: AtomicUsize::new(usize::MAX),
            branch: AtomicUsize::new(usize::MAX),
            vertex: AtomicUsize::new(usize::MAX),
            mutex: Mutex::new(()),
        }
    }

    pub fn to_parent(&self) -> Parent {
        let _lock = self.mutex.lock().unwrap();

        Parent {
            root: self.root.load(Ordering::SeqCst),
            branch: self.branch.load(Ordering::SeqCst),
            vertex: self.vertex.load(Ordering::SeqCst),
        }
    }

    /// Compare `self` to `other` withoth locking the object.
    ///
    /// If this function returns `true` than `self` is guaranteed to be
    /// smaller than `other`. If it returns `false` no guarantees can be
    /// made - it can be bigger, smaller or equal.
    ///
    /// This is possible because the values of a `Visited` object can only
    /// monothonically decrement. So if we observe that at some point in time
    /// the value was lower we can be sure that it currently is lower too,
    /// even if we are observing stale data.
    ///
    /// This method is useful to perform a quick check to see if more expensive
    /// operations (like `store_if_greater`) should even be attempted.
    pub fn weak_less_than(&self, other: &Parent) -> bool {
        match self.inner_cmp(other, Ordering::SeqCst) {
            cmp::Ordering::Less => true,
            _ => false,
        }
    }

    /// Stores the value of `other` in `self` if `self` is greater.
    ///
    /// This method needs to lock the value in order to perform the comparison
    /// and the write. As such it is a more expensive operation and should generally
    /// be attempted only if `weak_less_than` returned `false` previosly.
    pub fn store_if_greater(&self, other: &Parent) -> bool {
        let _lock = self.mutex.lock().unwrap();

        // because of the lock this is guaranteed to be correct
        match self.inner_cmp(other, Ordering::SeqCst) {
            cmp::Ordering::Greater => {
                self.root.store(other.root, Ordering::SeqCst);
                self.branch.store(other.branch, Ordering::SeqCst);
                self.vertex.store(other.vertex, Ordering::SeqCst);
                true
            },
            _ => false,
        }
    }

    fn inner_cmp(&self, other: &Parent, ordering: Ordering) -> cmp::Ordering {
        let root = self.root.load(ordering);
        match root.cmp(&other.root) {
            cmp::Ordering::Less => cmp::Ordering::Less,
            cmp::Ordering::Greater => cmp::Ordering::Greater,
            cmp::Ordering::Equal => {
                let branch = self.branch.load(ordering);
                match branch.cmp(&other.branch) {
                    cmp::Ordering::Less => cmp::Ordering::Less,
                    cmp::Ordering::Greater => cmp::Ordering::Greater,
                    cmp::Ordering::Equal => {
                        let vertex = self.vertex.load(ordering);
                        vertex.cmp(&other.vertex)
                    }
                }
            }
        }
    }
}
