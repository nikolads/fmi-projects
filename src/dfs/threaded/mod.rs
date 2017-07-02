use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, Thread};

use graph::Graph;
use thread_pool::Pool;
use self::visit::{Parent, Visited};

mod visit;

pub struct State<G: Graph + Send + Sync + 'static> {
    graph: Arc<G>,
    workers: Workers,
    parents: Vec<Visited>,
    pool: Arc<Mutex<Pool>>,
    wait_cnt: AtomicUsize,
    wait_thread: Thread,
}

impl<G: Graph + Send + Sync + 'static> State<G> {
    pub fn new(graph: &Arc<G>, pool: &Arc<Mutex<Pool>>) -> Self {
        State {
            graph: graph.clone(),
            workers: Workers::new(),
            parents: (0..graph.num_vertices()).into_iter().map(|_| Visited::max()).collect(),
            pool: pool.clone(),
            wait_cnt: AtomicUsize::new(0),
            wait_thread: thread::current(),
        }
    }

    pub fn run(this: &Arc<Self>) -> Vec<Parent> {
        {
            let tx = this.workers.get_or_spawn(this, 0);

            for v in this.graph.vertices() {
                tx.send(Msg::Vertex(v, Parent::new(v, v, 0))).unwrap();
            }
        }

        this.workers.pop(0);

        while this.wait_cnt.load(Ordering::SeqCst) != 0 {
            thread::park();
        }

        this.parents.iter().map(|visited| visited.to_parent()).collect()
    }
}

struct Workers {
    inner: Mutex<(VecDeque<Sender<Msg>>, usize)>,
}

impl Workers {
    pub fn new() -> Self {
        Workers {
            inner: Mutex::new((VecDeque::new(), 0)),
        }
    }

    pub fn spawn<G>(&self, state: &Arc<State<G>>, branch: usize) where
        G: Graph + Send + Sync + 'static
    {
        let mut lock = self.inner.lock().unwrap();
        let (ref mut vec, ref first) = *lock;
        let index = branch - first;

        for i in vec.len()..index+1 {
            let branch = i + first;
            let state_copy = state.clone();
            let (tx, rx) = mpsc::channel();

            state.pool.lock().unwrap().spawn(move || task_main(state_copy, rx, branch));

            state.wait_cnt.fetch_add(1, Ordering::SeqCst);
            vec.push_back(tx);
        }
    }

    pub fn pop(&self, branch: usize) {
        let mut lock = self.inner.lock().unwrap();
        let (ref mut vec, ref mut first) = *lock;
        assert!(branch == *first);

        *first = *first + 1;
        vec.pop_front();
    }

    pub fn get(&self, branch: usize) -> Option<Sender<Msg>> {
        let lock = self.inner.lock().unwrap();
        let (ref vec, ref first) = *lock;

        let index = branch - first;
        vec.get(index).cloned()
    }

    pub fn get_or_spawn<G>(&self, state: &Arc<State<G>>, branch: usize) -> Sender<Msg> where
        G: Graph + Send + Sync + 'static
    {
        match self.get(branch) {
            Some(sender) => sender,
            None => {
                self.spawn(state, branch);
                self.get(branch).unwrap()
            }
        }
    }
}

enum Msg {
    Vertex(usize, Parent),
}

fn task_main<G>(state: Arc<State<G>>, receiver: Receiver<Msg>, branch: usize) where
    G: Graph + Send + Sync + 'static
{
    let mut stack: Option<(usize, Parent)>;
    let mut cache = Cache::new(&state, branch);
    let mut loop_cnt = 0;

    for msg in receiver.iter() {
        match msg {
            Msg::Vertex(vert, parent) => {
                stack = Some((vert, parent));
            },
        }

        while let Some((vert, parent)) = stack.take() {
            loop_cnt += 1;

            if branch != parent.branch {
                println!("{} {}", branch, parent.branch);
            }

            if !state.parents[vert].weak_less_than(&parent) {
                let succeeded = state.parents[vert].store_if_greater(&parent);

                if succeeded {
                    for (i, nb) in state.graph.neighbours(vert).enumerate() {
                        let candidate = Parent::new(parent.root, vert, parent.branch + i);
                        let potentially_better = !state.parents[nb].weak_less_than(&candidate);

                        if potentially_better {
                            if i == 0 {
                                stack = Some((nb, candidate));
                            }
                            else {
                                cache.worker(candidate.branch).send(Msg::Vertex(nb, candidate)).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }

    state.workers.pop(branch + 1);

    let cnt = state.wait_cnt.fetch_sub(1, Ordering::SeqCst);
    if cnt == 1 {
        state.wait_thread.unpark();
    }

    LOOP_COUNTER.fetch_add(loop_cnt, Ordering::SeqCst);
}

struct Cache<G: Graph + Send + Sync + 'static> {
    tx_cache: Vec<Sender<Msg>>,
    first: usize,
    state: Arc<State<G>>,
}

impl<G: Graph + Send + Sync + 'static> Cache<G> {
    fn new(state: &Arc<State<G>>, first: usize) -> Self {
        Cache {
            tx_cache: Vec::new(),
            first: first + 1,
            state: state.clone(),
        }
    }

    fn worker(&mut self, branch: usize) -> &Sender<Msg> {
        for i in (self.tx_cache.len() + self.first)..(branch + 1) {
            let tx = self.state.workers.get_or_spawn(&self.state, i);
            self.tx_cache.push(tx);
        }

        if (branch - self.first) > 10000000 {
            println!("index: {} ({} - {})", branch - self.first, branch, self.first);
        }
        &self.tx_cache[branch - self.first]
    }
}

use std::sync::atomic::ATOMIC_USIZE_INIT;
pub static LOOP_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;
