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

                    answ_sender.send((i, graph)).unwrap();
                });
            }

            mem::drop(answ_sender);

            let mut graph = Vec::with_capacity(threads);
            unsafe { graph.set_len(threads) };

            for (i, part) in answ_receiver.iter() {
                graph[i] = part;
            }

            println!("{:?}", graph);

            /************************************************/
            use std::sync::{Arc, RwLock};
            use std::usize;

            let used = (0..n_verts).into_iter().map(|_| RwLock::new((usize::MAX, usize::MAX))).collect::<Vec<_>>();
            let used = Arc::new(used);

            let mut senders = Vec::with_capacity(threads);
            let mut receivers = Vec::with_capacity(threads);
            for _ in 0..threads {
                let (sender, receiver) = mpsc::channel();
                senders.push(sender);

                receivers.push(receiver);
            }

            for (i, (thread, recv)) in pool.threads().zip(receivers).enumerate() {
                let used = used.clone();
                let senders = senders.clone();

                thread.execute(move || dfs(i, recv, senders, used));
            }

            senders[0].send(Msg::Exit).unwrap();

            pool.join().unwrap();
        }
    }
}

use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, RwLock};

enum Msg {
    Node(usize, usize),
    Exit,
}

fn dfs(branch: usize, receiver: Receiver<Msg>, senders: Vec<Sender<Msg>>, used: Arc<Vec<RwLock<(usize, usize)>>>) {
    for msg in receiver.iter() {
        match msg {
            Msg::Node(vert, parent) => {
                {
                    let lock = used[vert].read().unwrap();
                    let (curr_parent, curr_branch) = *lock;

                    if curr_branch < branch {
                        continue;
                    }

                    if curr_branch == branch && curr_parent < parent {
                        continue;
                    }

                    // TODO
                }
            },
            Msg::Exit => {
                let _ = senders[branch+1].send(Msg::Exit);
                return;
            }
        }
    }
}
