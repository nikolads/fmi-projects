use std::mem;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, Thread};
use std::time::Instant;

use graph::Graph;
use thread_pool::Pool;

pub struct State<G: Graph + Send + Sync + 'static> {
    graph: Arc<G>,
    workers: Mutex<Vec<Sender<Msg>>>,
    parents: Vec<RwLock<Option<Parent>>>,
    pool: Arc<RwLock<Pool>>,
    wait_cnt: AtomicUsize,
    wait_thread: Thread,
}

impl<G: Graph + Send + Sync + 'static> State<G> {
    pub fn new(graph: &Arc<G>, pool: &Arc<RwLock<Pool>>) -> Self {
        State {
            graph: graph.clone(),
            workers: Mutex::new(Vec::new()),
            parents: (0..graph.num_vertices()).into_iter().map(|_| RwLock::new(None)).collect(),
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
        println!("enqueue: {}", ::format_dur(&ts_end.duration_since(ts_begin)));

        // TODO: hacky sollution
        {
            let (dummy, _) = mpsc::channel();
            this.workers.lock().unwrap()[0] = dummy;
        };

        while this.wait_cnt.load(Ordering::SeqCst) != 0 {
            thread::park();
        }

        this.parents.iter().map(|rwlock| rwlock.read().unwrap().clone()).collect()
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
                    tx_cache: Vec::new()
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

enum Msg {
    Vertex(usize, Parent),
}

struct Task<G: Graph + Send + Sync + 'static> {
    state: Arc<State<G>>,
    receiver: Receiver<Msg>,
    tx_cache: Vec<Sender<Msg>>,
}

impl<G: Graph + Send + Sync + 'static> Task<G> {
    fn main(self, branch: usize) {
        // println!("enter task_main {}", branch);
        let mut stack: Option<(usize, Parent)>;
        let mut loop_cnt = 0;

        for msg in self.receiver.iter() {
            match msg {
                Msg::Vertex(vert, parent) => {
                    stack = Some((vert, parent));
                },
            }

            while let Some((vert, parent)) = stack.take() {
                loop_cnt += 1;

                {
                    let mut lock = self.state.parents[vert].write().unwrap();

                    let should_write = match *lock {
                        Some(ref prev_parent) => parent < *prev_parent,
                        None => true
                    };

                    if should_write {
                        *lock = Some(parent.clone());
                    }
                }

                for (i, nb) in self.state.graph.neighbours(vert).enumerate() {
                    let parent_candidate = Parent::new(parent.root, vert, parent.branch + i);

                    let better_route = match *self.state.parents[nb].read().unwrap() {
                        Some(ref prev_parent) => parent_candidate < *prev_parent,
                        None => true,
                    };

                    if better_route {
                        if i == 0 {
                            stack = Some((nb, parent_candidate));
                        }
                        else {
                            State::worker(&self.state, parent_candidate.branch).send(Msg::Vertex(nb, parent_candidate)).unwrap();
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

        // println!("exit task_main {}", branch);

        LOOP_COUNTER.fetch_add(loop_cnt, Ordering::SeqCst);
    }

    fn worker(&mut self, branch: usize) -> &Sender<Msg> {
        for i in self.tx_cache.len()..branch {
            let tx = State::worker(&self.state, i);
            self.tx_cache.push(tx);
        }

        &self.tx_cache[branch]
    }
}

use std::sync::atomic::ATOMIC_USIZE_INIT;
pub static LOOP_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
