function colorMutate(c, probability) {
  var color = c;
  if (Math.random() < probability)
    color = color.replace(color[Math.floor(Math.random()*6 + 1)], Math.floor(Math.random()*16).toString(16));

  return color;
}

function colorCrossOver(cA, cB) {
  var color = "";
  if (cA == cB)
    color = "#" + cA.substr(4, 3) + cA.substr(1, 3);
  else if (Math.random() >=0.5)
    color = "#" + cA.substr(1, 3) + cB.substr(4, 3);
  else
    color = "#" + cB.substr(1, 3) + cA.substr(4, 3);

  return color;
}

var GraphBuilder = {
  RandomGraph: function(graph, nodes, edges) {
    var that = self;
    that.prefix = 'rand_';

    for (var i = 0; i < nodes; i++) {
      graph.addNodes(that.prefix + i);
    }

    for (var i=0; i < edges; i++) {
      var randS = Math.floor(Math.random()*nodes);
      var randT = Math.floor(Math.random()*nodes);
      while (randS == randT || (graph.adjacency[that.prefix + randS] != undefined && graph.adjacency[that.prefix + randS][that.prefix + randT] != undefined))
        randT = Math.floor(Math.random()*nodes);

      graph.addEdges(
        [that.prefix + randS, that.prefix + randT],
        [that.prefix + randT, that.prefix + randS]
      );
    }
  }

};

/**
* @param graph Reference to graph
* @param m0 The number of nodes in the seed network.
* @param m The number of edges to add at each time step.
* @param final_size The number of nodes in generated networks
*/
GraphBuilder.BarabasiAlbert = function (graph, m0, m, final_size) {
  var that = self;
  that.prefix = "ba__";

  that.graph = graph;
  that.m0 = m0;
  that.m = m;
  that.final_size = final_size;

  /** @param n Source Node */
  that.addPreferentialEdge = function (n) {
    var G = that.graph;
    var edges_sum = G.edges.length;
    var target = Math.floor(Math.random() * edges_sum);

    var target_node = G.edges[target].source;

    if (n.data.color == undefined) {
      n.data.color = colorMutate(target_node.data.color, 0.5);
    }

    G.addEdges(
      [n.id, target_node.id],
      [target_node.id, n.id]
    );
  }

  // initialize graph
  if (that.m0 > that.final_size)
    that.m0 = that.final_size;

  for (var i = 0; i < that.m0; i++) {
    var node = new Springy.Node(that.prefix + i, {
      label: that.prefix + i,
      color: '#' + Math.floor(Math.random()*16777215).toString(16),
      }
    );
    that.graph.addNode(node);
  }

  for (var i = 0; i < that.m0; i++) {
    for (var j = 0; j < that.m0; j++)
      that.graph.addEdges([that.prefix + i, that.prefix + j]);
  }

  var nodeInterval = setInterval(function() {
    node_size = that.graph.nodes.length;

    if (node_size > that.final_size) {
      clearInterval(nodeInterval);
      return;
    }

    var node = new Springy.Node(
                that.prefix + node_size, {label: that.prefix + node_size}
              );

    that.graph.addNode(node);

    // add m number of edges
    for (var m = 0; m < that.m; m++)
        that.addPreferentialEdge(node); // node_size is current id
  }, 5);


  return that;
};
