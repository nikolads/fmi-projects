var graph = new Springy.Graph();
// GraphBuilder.BarabasiAlbert(graph, 1, 2, 100, node => {node.data.state = "susceptible"});
GraphBuilder.BarabasiAlbert(graph, 1, 3, 262, function (node) {
  node.data.state = "susceptible";
  node.data.color = "#0000ff";
});
//GraphBuilder.RandomGraph(graph, 250, 230);

graph.nodes[0].data.state = "infected";
graph.nodes[0].data.recovery = 0.0;
graph.nodes[0].data.color = "#ff0000";

var sir = new Sir(graph, 0.012, 0.04);

jQuery(function(){
  var springy = jQuery('#sir_graph').springy({
    graph: graph,
    stiffness: 1,
    repulsion: 3,
    minEnergyThreshold: 0.01,
    //damping: 0.6
  });

  window.g_springy = springy;
  console.log(springy);
});
