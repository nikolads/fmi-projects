var graph = new Springy.Graph();
GraphBuilder.BarabasiAlbert(graph, 1, 1, 250);
//GraphBuilder.RandomGraph(graph, 250, 230);

jQuery(function(){
  var springy = jQuery('#sir_graph').springy({
    graph: graph,
    stiffness: 100,
    repulsion: 100,
    minEnergyThreshold: 0.01,
    //damping: 0.6
  });

  window.g_springy = springy;
  console.log(springy);
});
