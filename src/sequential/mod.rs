//! Single threaded (sequential) dfs algorithm

use graph::GraphPart;
use tree::Tree;

pub fn dfs<G: GraphPart, T: Tree>(graph: &G) -> Vec<T> {
    let mut result = Vec::new();
    let mut used = vec![false; graph.num_owned_vertices()];

    for root in graph.owned_vertices() {
        if used[root] {
            continue;
        }

        let mut tree = T::new(root);
        let mut stack = Vec::new();

        used[root] = true;

        for v in graph.neighbours(root) {
            stack.push((root, v));
        }

        while !stack.is_empty() {
            let (parent, top) = stack.pop().unwrap();

            if !used[top] {
                used[top] = true;

                tree.add(parent, top);

                for v in graph.neighbours(top) {
                    stack.push((top, v));
                }
            }
        }

        result.push(tree);
    }

    result
}
