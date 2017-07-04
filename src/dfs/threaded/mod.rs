use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use std::sync::mpsc::{self, Sender};
use std::thread::{self, Thread};
use std::usize;

use graph::Graph;
use thread_pool::Pool;
use tree::Tree;

#[derive(Debug)]
pub struct Dfs<G: Graph + Send + Sync + 'static> {
    inner: Arc<Inner<G>>
}

#[derive(Debug)]
pub struct Inner<G: Graph + Send + Sync + 'static> {
    graph: Arc<G>,
    owner: Vec<AtomicUsize>,
}

impl<G: Graph + Send + Sync + 'static> Dfs<G> {
    pub fn new(graph: &Arc<G>) -> Self {
        Dfs {
            inner: Arc::new(Inner {
                graph: graph.clone(),
                owner: (0..graph.num_vertices()).into_iter().map(|_| AtomicUsize::new(usize::MAX)).collect(),
            })
        }
    }

    pub fn run<T>(&mut self, pool: &Arc<Mutex<Pool>>) -> Vec<T> where
        T: Tree + Send + 'static,
    {
        let (answ_tx, answ_rx) = mpsc::channel();
        let wait_counter = Arc::new(AtomicUsize::new(0));

        {
            let pool_clone = pool.clone();
            let state = self.inner.clone();
            let wait_counter = wait_counter.clone();
            let thread = thread::current();

            wait_counter.fetch_add(1, Ordering::SeqCst);
            pool.lock().unwrap().spawn(|| spawner(state, 0, pool_clone, (wait_counter, thread), answ_tx));
        }

        answ_rx.iter().collect()

        // while wait_counter.load(Ordering::SeqCst) != 0 {
        //     thread::park();
        // }
    }
}

fn spawner<G, T>(state: Arc<Inner<G>>, root: usize, pool: Arc<Mutex<Pool>>, wait: (Arc<AtomicUsize>, Thread), answ_tx: Sender<T>) where
    G: Graph + Send + Sync + 'static,
    T: Tree + Send + 'static,
{
    let (ref wait_counter, ref thread) = wait;

    for next_root in root..state.owner.len() {
        if state.owner[next_root].compare_and_swap(usize::MAX, next_root, Ordering::SeqCst) == usize::MAX {
            let answ_tx = answ_tx.clone();
            let pool_clone = pool.clone();
            let state = state.clone();
            let wait = wait.clone();

            wait_counter.fetch_add(1, Ordering::SeqCst);
            pool.lock().unwrap().spawn(move || spawner(state, next_root, pool_clone, wait, answ_tx));
            break;
        }
    }

    task_main(state, root, answ_tx);

    if wait_counter.fetch_sub(1, Ordering::SeqCst) == 1 {
        thread.unpark();
    }
}

struct Owned<G: Graph + Send + Sync + 'static> {
    root: usize,
    data: Vec<bool>,
    state: Arc<Inner<G>>,
}

impl<G: Graph + Send + Sync + 'static> Owned<G> {
    pub fn new(state: &Arc<Inner<G>>, root: usize) -> Self {
        Owned {
            root: root,
            data: vec![false; state.graph.num_vertices()],
            state: state.clone(),
        }
    }

    pub fn acquire(&mut self, vert: usize) -> bool {
        if !self.data[vert] {
            self.data[vert] =
                self.state.owner[vert].compare_and_swap(usize::MAX, self.root, Ordering::SeqCst) == usize::MAX;
        }

        self.data[vert]
    }
}

fn task_main<G, T>(state: Arc<Inner<G>>, root: usize, answ_tx: Sender<T>) where
    G: Graph + Send + Sync + 'static,
    T: Tree,
{
    let mut loop_cnt = 0;
    let mut stack = Vec::new();
    let mut used = vec![false; state.graph.num_vertices()];
    let mut owned = Owned::new(&state, root);
    let mut result = T::new(root);

    used[root] = true;

    for v in state.graph.neighbours(root) {
        if owned.acquire(v) {
            stack.push((root, v));
        }
        else {
            used[v] = true;
        }
    }

    while !stack.is_empty() {
        loop_cnt += 1;
        let (parent, vert) = stack.pop().unwrap();

        if !used[vert] {
            used[vert] = true;
            result.add(vert, parent);

            for child in state.graph.neighbours(vert) {
                if !used[child] {
                    if owned.acquire(child) {
                        stack.push((vert, child));
                    }
                    else {
                        used[child] = true;
                    }
                }
            }
        }
    }

    answ_tx.send(result).unwrap();
    BENCH_EDGE_COUNT.fetch_add(loop_cnt, Ordering::SeqCst);
}

pub static BENCH_EDGE_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;
