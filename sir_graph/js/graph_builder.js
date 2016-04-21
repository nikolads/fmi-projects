var GraphBuilder = {

  BarabasiAlbert: function(graph, init_num_nodes, final_num_nodes) {
    if (init_num_nodes > final_num_nodes)
      init_num_nodes = final_num_nodes;

    for (var i = 0; i < init_num_nodes; i++) {
      graph.addNodes('ba_' + i);
    }

    for (var i = 0; i < init_num_nodes; i++) {
      for (var j = 0; j < init_num_nodes; j++)
        graph.addEdges(['ba_' + i, 'ba_' + j, {color: '#000'}]);
    }
    return;

    for (var i = init_num_nodes; i < final_num_nodes; i++) {
      var node = new Springy.Node("ba_" + i, {label: "ba_" + i});
      graph.addNode(node);

      var target = Math.random();

      // connect to other
      for (var j = 0; j < graph.nodes.length; j++) {
        var prob = 2*graph.getNodeDegree(graph.nodes[j]) / graph.edges.length;

        if (target < prob) {
          graph.addEdges(
            ["ba_" + i, "ba_" + j, {color: '#000'}],
            ["ba_" + j, "ba_" + i, {color: '#000'}]
          );
        }

      }

    }

    return graph;
  }

};
