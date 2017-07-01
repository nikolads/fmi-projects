use std::mem;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, Thread};
use std::time::Instant;
use std::usize;

use graph::Graph;
use thread_pool::Pool;

pub struct State<G: Graph + Send + Sync + 'static> {
    graph: Arc<G>,
    workers: Mutex<Vec<Sender<Msg>>>,
    parents: Vec<Visited>,
    // parents: Vec<RwLock<Option<Parent>>>,
    pool: Arc<RwLock<Pool>>,
    wait_cnt: AtomicUsize,
    wait_thread: Thread,
}

impl<G: Graph + Send + Sync + 'static> State<G> {
    pub fn new(graph: &Arc<G>, pool: &Arc<RwLock<Pool>>) -> Self {
        State {
            graph: graph.clone(),
            workers: Mutex::new(Vec::new()),
            parents: (0..graph.num_vertices()).into_iter().map(|_| Visited::max()).collect(),
            // parents: (0..graph.num_vertices()).into_iter().map(|_| RwLock::new(None)).collect(),
            pool: pool.clone(),
            wait_cnt: AtomicUsize::new(0),
            wait_thread: thread::current(),
        }
    }

    pub fn run(this: &Arc<Self>) -> Vec<Option<Parent>> {
        let ts_begin = ::std::time::Instant::now();

        {
            Self::spawn_worker(this, 0);
            let tx = Self::worker(this, 0);

            for v in this.graph.vertices() {
                tx.send(Msg::Vertex(v, Parent::new(v, v, 0))).unwrap();
            }
        }

        let ts_end = ::std::time::Instant::now();
        // // println!("enqueue: {}", ::format_dur(&ts_end.duration_since(ts_begin)));

        // TODO: hacky sollution
        {
            let (dummy, _) = mpsc::channel();
            this.workers.lock().unwrap()[0] = dummy;
        };

        while this.wait_cnt.load(Ordering::SeqCst) != 0 {
            // // println!("park");
            thread::park();
        }

        // // println!("unpark");

        // this.parents.iter().map(|rwlock| rwlock.read().unwrap().clone()).collect();
        this.parents.iter().map(|visited| Some(visited.to_parent())).collect()
    }

    fn spawn_worker(this: &Arc<Self>, branch: usize) {
        let mut lock = this.workers.lock().unwrap();

        if branch == lock.len() {
            let state = this.clone();
            let (tx, rx) = mpsc::channel();

            this.pool.write().unwrap().spawn(move || {
                let task = Task {
                    state: state,
                    receiver: rx,
                };

                task.main(branch);
            });

            this.wait_cnt.fetch_add(1, Ordering::SeqCst);
            lock.push(tx);
        }
        else if branch > lock.len() {
            // TODO: fix
            mem::drop(lock);
            Self::spawn_worker(this, branch - 1);
            Self::spawn_worker(this, branch);
        }
    }

    fn worker(this: &Arc<Self>, branch: usize) -> Sender<Msg> {
        if let Some(tx) = this.workers.lock().unwrap().get(branch) {
            return tx.clone();
        }

        Self::spawn_worker(this, branch);
        this.workers.lock().unwrap()[branch].clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Parent {
    root: usize,
    branch: usize,
    vertex: usize,
}

impl Parent {
    fn new(root: usize, vertex: usize, branch: usize) -> Self {
        Parent {
            root: root,
            vertex: vertex,
            branch: branch,
        }
    }
}

struct Visited {
    root: AtomicUsize,
    branch: AtomicUsize,
    vertex: AtomicUsize,
    mutex: Mutex<()>,
}

impl Visited {
    fn from_parent(parent: &Parent) -> Self {
        Visited {
            root: AtomicUsize::new(parent.root),
            branch: AtomicUsize::new(parent.branch),
            vertex: AtomicUsize::new(parent.vertex),
            mutex: Mutex::new(()),
        }
    }

    fn max() -> Self {
        Visited {
            root: AtomicUsize::new(usize::MAX),
            branch: AtomicUsize::new(usize::MAX),
            vertex: AtomicUsize::new(usize::MAX),
            mutex: Mutex::new(()),
        }
    }

    pub fn weak_less_than(&self, other: &Parent) -> bool {
        let root = self.root.load(Ordering::SeqCst);
        // println!("root {:?}", root);
        if root != other.root {
            return root < other.root;
        }

        let branch = self.branch.load(Ordering::SeqCst);
        // println!("branch {:?}", branch);
        if branch != other.branch {
            return branch < other.branch;
        }

        let vertex = self.vertex.load(Ordering::SeqCst);
        // println!("vertex {:?}", vertex);
        vertex < other.vertex
    }

    fn greater_than(&self, other: &Parent) -> bool {
        let root = self.root.load(Ordering::SeqCst);
        // println!("g_root {:?}", root);
        if root != other.root {
            return root > other.root;
        }

        let branch = self.branch.load(Ordering::SeqCst);
        // println!("g_branch {:?}", branch);
        if branch != other.branch {
            return branch > other.branch;
        }

        let vertex = self.vertex.load(Ordering::SeqCst);
        // println!("g_vertex {:?}", vertex);
        vertex > other.vertex
    }

    fn swap_if_greater(&self, other: &Parent) -> bool {
        // println!("swap({:?})", other);
        let _lock = self.mutex.lock().unwrap();

        // because of the lock this is guaranteed to be correct
        // if !self.weak_less_than(other) {
        if self.greater_than(other) {
            self.root.store(other.root, Ordering::SeqCst);
            self.branch.store(other.branch, Ordering::SeqCst);
            self.vertex.store(other.vertex, Ordering::SeqCst);

            // println!("swap succeeded");
            true
        }
        else {
            // println!("swap failed");
            false
        }
    }

    fn to_parent(&self) -> Parent {
        let _lock = self.mutex.lock().unwrap();

        Parent {
            root: self.root.load(Ordering::SeqCst),
            branch: self.branch.load(Ordering::SeqCst),
            vertex: self.vertex.load(Ordering::SeqCst),
        }
    }
}

enum Msg {
    Vertex(usize, Parent),
}

struct Task<G: Graph + Send + Sync + 'static> {
    state: Arc<State<G>>,
    receiver: Receiver<Msg>,
}

struct Cache<'a, G: Graph + Send + Sync + 'static> {
    tx_cache: Vec<Sender<Msg>>,
    first: usize,
    task: &'a Task<G>,
}

impl<G: Graph + Send + Sync + 'static> Task<G> {
    fn main(self, branch: usize) {
        // // println!("enter task_main {}", branch);
        let mut stack: Option<(usize, Parent)>;
        let mut cache = Cache::new(&self, branch);
        let mut loop_cnt = 0;

        for msg in self.receiver.iter() {
            match msg {
                Msg::Vertex(vert, parent) => {
                    stack = Some((vert, parent));
                },
            }

            while let Some((vert, parent)) = stack.take() {
                loop_cnt += 1;
                // println!("[{:?}] {:?} {:?}", branch, vert, parent);

                {
                    // println!("less_than {:?} vs {:?}", vert, parent);
                    if !self.state.parents[vert].weak_less_than(&parent) {
                        // println!("false");
                        let succeeded = self.state.parents[vert].swap_if_greater(&parent);

                        if !succeeded {
                            continue;
                        }
                    }
                    else {
                        continue;
                    }
                    // let mut lock = self.state.parents[vert].write().unwrap();

                    // let should_write = match *lock {
                    //     Some(ref prev_parent) => parent < *prev_parent,
                    //     None => true
                    // };

                    // if should_write {
                    //     *lock = Some(parent.clone());
                    // }
                    // else {
                    //     continue;
                    // }
                }

                for (i, nb) in self.state.graph.neighbours(vert).enumerate() {
                    let parent_candidate = Parent::new(parent.root, vert, parent.branch + i);

                    // let better_route = match *self.state.parents[nb].read().unwrap() {
                    //     Some(ref prev_parent) => parent_candidate < *prev_parent,
                    //     None => true,
                    // };

                    let better_route = !self.state.parents[nb].weak_less_than(&parent_candidate);
                    // println!("is_less {:?} vs {:?}: {:?}", nb, parent_candidate, !better_route);

                    if better_route {
                        if i == 0 {
                            stack = Some((nb, parent_candidate));
                        }
                        else {
                            // State::worker(&self.state, parent_candidate.branch).send(Msg::Vertex(nb, parent_candidate)).unwrap();
                            cache.worker(parent_candidate.branch).send(Msg::Vertex(nb, parent_candidate)).unwrap();
                        }
                    }
                }
            }
        }

        // TODO: hacky sollution
        {
            let mut lock = self.state.workers.lock().unwrap();
            if lock.len() > branch + 1 {
                let (dummy, _) = mpsc::channel();
                lock[branch + 1] = dummy;
            }
        };

        let cnt = self.state.wait_cnt.fetch_sub(1, Ordering::SeqCst);
        if cnt == 1 {
            self.state.wait_thread.unpark();
        }

        // // println!("exit task_main {}", branch);

        LOOP_COUNTER.fetch_add(loop_cnt, Ordering::SeqCst);
    }
}

impl<'a, G: Graph + Send + Sync + 'static> Cache<'a, G> {
    fn new(task: &'a Task<G>, first: usize) -> Self {
        Cache {
            tx_cache: Vec::new(),
            first: first + 1,
            task: task,
        }
    }

    fn worker(&mut self, branch: usize) -> &Sender<Msg> {
        for i in (self.tx_cache.len() + self.first)..(branch + 1) {
            let tx = State::worker(&self.task.state, i);
            self.tx_cache.push(tx);
        }

        &self.tx_cache[branch - self.first]
    }
}

use std::sync::atomic::ATOMIC_USIZE_INIT;
pub static LOOP_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
