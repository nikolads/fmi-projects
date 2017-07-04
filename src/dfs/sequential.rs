use graph::Graph;
use tree::Tree;

pub fn dfs<G: Graph, T: Tree>(graph: &G) -> Vec<T> {
    let mut result = Vec::new();
    let mut used = vec![false; graph.num_vertices()];
    let mut stack = Vec::new();

    let mut loop_cnt = 0;

    for root in graph.vertices() {
        if used[root] {
            continue;
        }

        let mut tree = T::new(root);

        for v in graph.neighbours(root) {
            if !used[v] {
                stack.push((root, v));
            }
        }

        while !stack.is_empty() {
            loop_cnt += 1;
            let (parent, vert) = stack.pop().unwrap();

            if !used[vert] {
                used[vert] = true;
                tree.add(parent, vert);

                for child in graph.neighbours(vert) {
                    if !used[child] {
                        stack.push((vert, child));
                    }
                }
            }
        }

        result.push(tree);
    }

    println!("loop count: {}", loop_cnt);
    result
}
