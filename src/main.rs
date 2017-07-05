#![allow(dead_code)]

#![feature(conservative_impl_trait)]
#![feature(fnbox)]
#![feature(range_contains)]

#[macro_use(crate_version)]
extern crate clap;

use std::fs::File;
use std::time::{Duration, Instant};

use graph::adj_lists::AdjLists;
use thread_pool::Pool;
use tree::ParentArray;

pub mod args;
pub mod dfs;
pub mod graph;
pub mod thread_pool;
pub mod tree;

fn main() {
    let matches = args::parse();

    let threads = matches.value_of("threads").map_or(1, |threads| threads.parse::<usize>().unwrap());
    println!("Running on {} threads", threads);

    let n_verts = matches.value_of("vertices").unwrap().parse::<usize>().unwrap();
    let n_edges = matches.value_of("edges").unwrap().parse::<usize>().unwrap();

    if threads == 0 {
        use dfs::sequential;

        let ts_begin = Instant::now();

        let graph = match matches.value_of("input") {
            Some(input) => AdjLists::from_file(File::open(input).unwrap()).unwrap(),
            None => generate_graph_directed_st(n_verts, n_edges),
        };
        let ts_generate = Instant::now();

        let forest = sequential::dfs::<_, ParentArray>(&graph);
        let ts_dfs = Instant::now();

        if n_verts <= 100 {
            println!("graph: {:?}", graph);
            println!("forest: {:?}", forest);
        }

        println!("generate: {}", format_dur(&ts_generate.duration_since(ts_begin)));
        println!("dfs: {}", format_dur(&ts_dfs.duration_since(ts_generate)));
    }
    else {
        use std::sync::{Arc, Mutex};
        use dfs::threaded::Dfs;

        let mut pool = Pool::new(threads as usize).unwrap();

        let ts_begin = Instant::now();

        let graph = match matches.value_of("input") {
            Some(input) => AdjLists::from_file(File::open(input).unwrap()).unwrap(),
            None => generate_graph_directed_mt(threads, n_verts, n_edges, &mut pool),
        };
        let graph = Arc::new(graph);
        let ts_generate = Instant::now();

        let pool = Arc::new(Mutex::new(pool));
        let mut dfs = Dfs::new(&graph);

        let vec = dfs.run::<ParentArray>(&pool);
        let ts_dfs = Instant::now();

        if n_verts <= 100 {
            println!("{:?}", graph);
            println!("{:?}", vec);
        }

        println!("trees: {}", vec.len());
        println!("edges visited: {}", dfs::threaded::BENCH_EDGE_COUNT.load(::std::sync::atomic::Ordering::SeqCst));
        println!("tasks: {}", dfs::threaded::BENCH_TASK_COUNT.load(::std::sync::atomic::Ordering::SeqCst));
        println!("generate: {}", format_dur(&ts_generate.duration_since(ts_begin)));
        println!("dfs: {}", format_dur(&ts_dfs.duration_since(ts_generate)));
    }
}

fn generate_graph_directed_st(n_verts: usize, n_edges: usize) -> AdjLists {
    use std::iter;
    use graph::adj_lists::Part;

    let mut part =  Part::new(0..n_verts, 0..n_verts);
    part.generate_edges_directed(n_edges, None);

    AdjLists::from_parts(n_verts, iter::once(part))
}

fn generate_graph_directed_mt(threads: usize, n_verts: usize, n_edges: usize, pool: &mut Pool) -> AdjLists {
    use std::mem;
    use std::sync::mpsc;

    let (answ_tx, answ_rx) = mpsc::sync_channel(threads as usize);

    for i in 0..threads {
        let answ_tx = answ_tx.clone();

        let vert_per_thread = n_verts / threads;
        let owned = (i * vert_per_thread)..((i+1) * vert_per_thread);
        let target = 0..n_verts;
        let edges = n_edges / threads;

        pool.spawn(move || {
            use graph::adj_lists::Part;

            let mut part = Part::new(owned, target);
            part.generate_edges_directed(edges, None);

            answ_tx.send(part).unwrap();
        });
    }

    mem::drop(answ_tx);

    AdjLists::from_parts(n_verts, answ_rx.iter())
}

fn format_dur(dur: &Duration) -> f64 {
    dur.as_secs() as f64 + dur.subsec_nanos() as f64 * 1e-9
}
