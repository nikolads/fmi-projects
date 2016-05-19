// var sir = new Sir(graph, 0.012, 0.04);

jQuery(function(){
   document.getElementById("sir_graph").width = window.innerWidth;
   document.getElementById("sir_graph").height = window.innerHeight;
});

window.onload = function() {
  var gui = new dat.GUI();
  var sirg = new SirGUIWrap(gui);
  var grapg = new GraphGUIWrap(gui);

  grapg.init_m = 3;
  grapg.step_m = 3;
  grapg.max_nodes = 262;

  grapg.rebuild(grapg.init_m, grapg.step_m, grapg.max_nodes);
};
