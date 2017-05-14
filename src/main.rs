#[macro_use(crate_version)]
extern crate clap;
extern crate rand;

use rand::{StdRng, Rng, SeedableRng};
use std::collections::HashSet;
use std::cmp;

pub mod args;
pub mod graph;
pub mod sequential;
pub mod tree;

fn main() {
    let matches = args::parse();

    let threads = matches.value_of("threads").map_or(1, |threads| threads.parse::<u32>().unwrap());
    println!("Running on {} threads", threads);

    if threads == 1 {
        use graph::AdjLists;
        use tree::ParentArray;

        let mut graph =  AdjLists::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(1, 4);
        graph.add_edge(1, 0);
        graph.add_edge(2, 1);
        graph.add_edge(3, 2);
        graph.add_edge(4, 3);
        graph.add_edge(4, 1);

        let forest: Vec<ParentArray> = sequential::dfs(&graph);

        println!("{:?}", forest);
    }
    else {
        unimplemented!();
    }
}
