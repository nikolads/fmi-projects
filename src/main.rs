#![allow(dead_code)]

#![feature(conservative_impl_trait)]
#![feature(fnbox)]
#![feature(range_contains)]

#[macro_use(crate_version)]
extern crate clap;

pub mod args;
pub mod graph;
pub mod sequential;
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
            use graph::AdjLists;
            use tree::ParentArray;

            let mut graph =  AdjLists::new(0..n_verts, 0..n_verts);
            graph.generate_edges_directed(n_edges, None);

            println!("graph: {:?}", graph);

            let forest = sequential::dfs::<_, ParentArray>(&graph);
            println!("{:?}", forest);
        }
        else {
            use thread_pool::Pool;
            use std::mem;
            use std::sync::mpsc;

            let threads = threads as usize;
            let pool = Pool::new(threads).unwrap();

            let (answ_sender, answ_receiver) = mpsc::sync_channel(threads);

            for (i, thread) in pool.threads().enumerate() {
                let answ_sender = answ_sender.clone();

                let vert_per_thread = n_verts / threads;
                let owned = (i * vert_per_thread)..((i+1) * vert_per_thread);
                let target = 0..n_verts;
                let edges = n_edges / threads;

                thread.execute(move || {
                    use graph::AdjLists;

                    let mut graph = AdjLists::new(owned, target);
                    graph.generate_edges_directed(edges, None);

                    answ_sender.send(graph).unwrap();
                });
            }

            mem::drop(answ_sender);

            for graph in answ_receiver.iter() {
                println!("{:?}", graph);
            }

            pool.join().unwrap();
        }
    }
}
