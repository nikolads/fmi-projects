#![allow(dead_code)]

#![feature(conservative_impl_trait)]
#![feature(fnbox)]
#![feature(range_contains)]

#[macro_use(crate_version)]
extern crate clap;

pub mod args;
pub mod graph;
pub mod thread_pool;
pub mod tree;

fn main() {
    let matches = args::parse();

    let threads = matches.value_of("threads").map_or(1, |threads| threads.parse::<u32>().unwrap());
    println!("Running on {} threads", threads);

    if let Some(_input) = matches.value_of("input") {
        unimplemented!();
    }
    else {
        let n_verts = matches.value_of("vertices").unwrap().parse::<usize>().unwrap();
        let n_edges = matches.value_of("edges").unwrap().parse::<usize>().unwrap();

        if threads == 1 {
            use std::iter;
            use graph::adj_lists::{AdjLists, Part};

            let n_verts = n_verts as u32;

            let mut part =  Part::new(0..n_verts, 0..n_verts);
            part.generate_edges_directed(n_edges, None);
            let graph = AdjLists::from_parts(n_verts, iter::once(part));

            println!("graph: {:?}", graph);
        }
        else {
            use std::mem;
            use std::sync::mpsc;
            use graph::adj_lists::AdjLists;
            use thread_pool::Pool;

            let n_verts = n_verts as u32;
            let n_edges = n_edges as u32;
            let mut pool = Pool::new(threads as usize).unwrap();

            let (answ_tx, answ_rx) = mpsc::sync_channel(threads as usize);

            for i in 0..threads {
                let answ_tx = answ_tx.clone();

                let vert_per_thread = n_verts / threads;
                let owned = (i * vert_per_thread)..((i+1) * vert_per_thread);
                let target = 0..n_verts;
                let edges = (n_edges / threads) as usize;

                pool.spawn(move || {
                    use graph::adj_lists::Part;

                    let mut part = Part::new(owned, target);
                    part.generate_edges_directed(edges, None);

                    answ_tx.send(part).unwrap();
                });
            }

            mem::drop(answ_tx);

            let graph = AdjLists::from_parts(n_verts, answ_rx.iter());
            println!("{:?}", graph);
        }
    }
}
