// global graph variable
var graph;
var sir;

jQuery(document).on('barabasi_finished', function(event) {
  for (i = 0; i < 5; i++) {
    var target = Math.floor(Math.random() * graph.nodes.length)
    makeInfected(graph.nodes[target]);
  }
  sir.start(100);
});

jQuery(function(){
   document.getElementById("sir_graph").width = window.innerWidth;
   document.getElementById("sir_graph").height = window.innerHeight;
});

window.onload = function() {
  var gui = new dat.GUI();
  var sirg = new SirGUIWrap(gui);
  var grapg = new GraphGUIWrap(gui);

  graph = grapg.graph;
  sir = new Sir(graph, 0.012, 0.04);

  grapg.init_m = 3;
  grapg.step_m = 3;
  grapg.max_nodes = 764;
  grapg.notify();

  grapg.rebuild(grapg.init_m, grapg.step_m, grapg.max_nodes);
};
