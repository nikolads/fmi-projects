//! Single threaded (sequential) dfs algorithm

use graph::Graph;
use tree::Tree;

pub fn dfs<G: Graph, T: Tree>(graph: &G) -> Vec<T> {
    let mut result = Vec::new();
    let mut used = vec![false; graph.num_vertices()];

    for root in graph.vertices() {
        if used[root] {
            continue;
        }

        let mut tree = T::new(root);
        let mut stack = Vec::new();

        used[root] = true;

        for v in graph.neighbours(root).rev() {
            stack.push((root, v));
        }

        while !stack.is_empty() {
            let (parent, top) = stack.pop().unwrap();

            if !used[top] {
                used[top] = true;

                tree.add(parent, top);

                for v in graph.neighbours(top).rev() {
                    stack.push((top, v));
                }
            }
        }

        result.push(tree);
    }

    result
}
