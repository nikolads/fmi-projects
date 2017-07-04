use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use std::sync::mpsc::{self, Sender};
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
        let answ_rx = {
            let (answ_tx, answ_rx) = mpsc::channel();
            let pool_clone = pool.clone();
            let state = self.inner.clone();

            self.inner.owner[0].store(0, Ordering::SeqCst);

            pool.lock().unwrap().spawn(|| spawner(state, 0, pool_clone, answ_tx));

            answ_rx
        };

        let res: Vec<T> = answ_rx.iter().collect();

        let ts_before = ::std::time::Instant::now();
        let forest = res.iter().filter_map(|tree| {
            let root = tree.root();

            if self.inner.owner[root].load(Ordering::SeqCst) == root {
                let mut res = T::new(root);

                for (vert, parent) in tree.iter() {
                    if self.inner.owner[vert].load(Ordering::SeqCst) == root {
                        res.add(vert, parent);
                    }
                }

                Some(res)
            }
            else {
                None
            }
        }).collect();

        let ts_after = ::std::time::Instant::now();
        println!("reformat {}", ::format_dur(&ts_after.duration_since(ts_before)));

        forest
    }
}

fn spawner<G, T>(state: Arc<Inner<G>>, root: usize, pool: Arc<Mutex<Pool>>, answ_tx: Sender<T>) where
    G: Graph + Send + Sync + 'static,
    T: Tree + Send + 'static,
{
    for next_root in (root + 1)..state.owner.len() {
        if acquire(&state.owner[next_root], next_root) {
            let answ_tx = answ_tx.clone();
            let pool_clone = pool.clone();
            let state = state.clone();

            pool.lock().unwrap().spawn(move || spawner(state, next_root, pool_clone, answ_tx));
            break;
        }
    }

    task_main(state, root, answ_tx);
}

struct Owned<G: Graph + Send + Sync + 'static> {
    root: usize,
    data: Vec<bool>,
    state: Arc<Inner<G>>,
}

impl<G: Graph + Send + Sync + 'static> Owned<G> {
    pub fn new(state: &Arc<Inner<G>>, root: usize) -> Self {
        let mut data = vec![false; state.graph.num_vertices()];
        data[root] = true;

        Owned {
            root: root,
            data: data,
            state: state.clone(),
        }
    }

    pub fn check(&mut self, vert: usize) -> bool {
        if !self.data[vert] {
            self.data[vert] = acquire(&self.state.owner[vert], self.root);
        }

        self.data[vert]
    }
}

fn acquire(owner: &AtomicUsize, root: usize) -> bool {
    let mut prev = owner.load(Ordering::SeqCst);

    while prev > root {
        let found = owner.compare_and_swap(prev, root, Ordering::SeqCst);
        if found == prev {
            return true;
        }

        prev = found;
        // backoff ?
    }

    false
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
        if owned.check(v) {
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
                    if owned.check(child) {
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
