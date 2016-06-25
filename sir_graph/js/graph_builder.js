function colorMutate(c, probability) {
    var color = c;
    if (Math.random() < probability)
        color = color.replace(color[Math.floor(Math.random() * 6 + 1)], Math.floor(Math.random() * 16).toString(16));

    return color;
}

function colorCrossOver(cA, cB) {
    var color = "";
    if (cA == cB)
        color = "#" + cA.substr(4, 3) + cA.substr(1, 3);
    else if (Math.random() >= 0.5)
        color = "#" + cA.substr(1, 3) + cB.substr(4, 3);
    else
        color = "#" + cB.substr(1, 3) + cA.substr(4, 3);

    return color;
}

var GraphBuilder = {
    RandomGraph: function(graph, nodes, edges, new_node_callback) {
        var that = self;
        that.prefix = 'rand_';

        for (var i = 0; i < nodes; i++) {
            var node = new Springy.Node(that.prefix + i, {
                label: that.prefix + i
            });
            if (new_node_callback !== undefined) {
                new_node_callback(node);
            }

            that.graph.addNode(node);
        }

        for (var i = 0; i < edges; i++) {
            var randS = Math.floor(Math.random() * nodes);
            var randT = Math.floor(Math.random() * nodes);
            while (randS == randT || (graph.adjacency[that.prefix + randS] != undefined && graph.adjacency[that.prefix + randS][that.prefix + randT] != undefined))
                randT = Math.floor(Math.random() * nodes);

            graph.addEdges(
                [that.prefix + randS, that.prefix + randT], [that.prefix + randT, that.prefix + randS]
            );

            var node = graph.nodeSet[that.prefix + randS];
            node.data.mass = 2 + 2 * Math.log(graph.getNodeDegree(node));

            node = graph.nodeSet[that.prefix + randT];
            node.data.mass = 2 + 2 * Math.log(graph.getNodeDegree(node));
        }
        jQuery(document).trigger('random_finished');
    }

};

/**
 * @param graph Reference to graph
 * @param m0 The number of nodes in the seed network.
 * @param m The number of edges to add at each time step.
 * @param final_size The number of nodes in generated networks
 */
GraphBuilder.BarabasiAlbert = function(graph, m0, m, final_size, new_node_callback) {
    var that = this;
    that.prefix = "ba__";

    that.graph = graph;
    that.m0 = m0;
    that.m = m;
    that.final_size = final_size;

    /**
     * @param n Source Node
     * @param count Number of edges to add
     */
    that.addPreferentialEdge = function(n, count) {
        var G = that.graph;
        var edges_sum = G.edges.length;
        var addedEdges = {};

        for (i = 0; i < count;) {
            var target = Math.floor(Math.random() * edges_sum);
            var target_node = G.edges[target].source;

            // make sure we are not adding a duplicate edge
            if (addedEdges[target_node.id] !== undefined) {
                continue;
            }

            if (n.data.color == undefined) {
                n.data.color = colorMutate(target_node.data.color, 0.5);
            }

            G.addEdges(
                [n.id, target_node.id], [target_node.id, n.id]
            );

            n.data.mass = 2 + 2 * Math.log(graph.getNodeDegree(n));
            target_node.data.mass = 2 + 2 * Math.log(graph.getNodeDegree(target_node));

            addedEdges[target_node.id] = 1;
            i++;
        }
    }

    // initialize graph
    if (that.m0 > that.final_size)
        that.m0 = that.final_size;

    for (var i = 0; i < that.m0; i++) {
        var node = new Springy.Node(that.prefix + i, {
            label: that.prefix + i,
            color: '#' + Math.floor(Math.random() * 16777215).toString(16),
        });

        if (new_node_callback !== undefined) {
            new_node_callback(node);
        }
        that.graph.addNode(node);
    }

    for (var i = 0; i < that.m0; i++) {
        for (var j = 0; j < that.m0; j++) {
            that.graph.addEdges([that.prefix + i, that.prefix + j]);

            var node = graph.nodeSet[that.prefix + i];
            node.data.mass = 2 + 2 * Math.log(graph.getNodeDegree(node));

            node = graph.nodeSet[that.prefix + j];
            node.data.mass = 2 + 2 * Math.log(graph.getNodeDegree(node));
        }
    }

    var nodeInterval = setInterval(function() {
        node_size = that.graph.nodes.length;

        if (node_size > that.final_size) {
            clearInterval(nodeInterval);
            jQuery(document).trigger('barabasi_finished');
            return;
        }

        var node = new Springy.Node(that.prefix + node_size, {
            label: that.prefix + node_size
        });
        if (new_node_callback !== undefined) {
            new_node_callback(node);
        }

        that.graph.addNode(node);

        // add m number of edges
        that.addPreferentialEdge(node, that.m); // node_size is current id
    }, 5);

    return that;
};

GraphBuilder.Tree = function(graph, tree_graph, root) {
    var that = this;
    that.prefix = "tree__";

    tree_graph.addNodes.apply(tree_graph, Object.keys(graph.adjacency));

    var visited = new Set();
    var Q = [];
    visited.add(root);
    Q.push(root);
    tree_graph.nodeSet[root].data.distance = 0;

    while (Q.length > 0) {
        var current = Q.shift();
        tree_graph.nodeSet[current].data.color = graph.nodeSet[current].data.color
        tree_graph.nodeSet[current].data.state = graph.nodeSet[current].data.state

        for (key in graph.adjacency[current]) {
            if (visited.has(key) == false) {
                Q.push(key)
                visited.add(key);
                tree_graph.nodeSet[key].data.distance = tree_graph.nodeSet[current].data.distance + 1;
                tree_graph.addEdges([current, key]);
            }
        }
    }
    tree_graph.nodeSet[root].data.color = "#F44336";
    return this;
};
