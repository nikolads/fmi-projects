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
            unimplemented!();
        }
    }
}
