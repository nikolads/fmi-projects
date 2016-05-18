var graph = new Springy.Graph();
GraphBuilder.BarabasiAlbert(graph, 3, 1, 200);

jQuery(function(){

   document.getElementById("sir_graph").width = window.innerWidth;
   document.getElementById("sir_graph").height = window.innerHeight;

  var springy = jQuery('#sir_graph').springy({
    graph: graph,
    stiffness: 10,
    repulsion: 10,
    minEnergyThreshold: 0.001,
    //damping: 0.6
  });

  window.g_springy = springy;
  console.log(springy);
});
