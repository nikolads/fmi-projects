var SirGUIWrap = function() {
  this.recovery_rate = 0.5;
  this.infection_rate = 0.5;
  this.start = false;
};

var GraphGUIWrap = function(graph) {
  this.init_m = 1;
  this.step_m = 1;
  this.max_nodes = 100;

  this.graph = graph;

  this.rebuild = function(init_m, step_m, max_nodes) {
    this.graph.nodeSet = {};
		this.graph.nodes = [];
		this.graph.edges = [];
		this.graph.adjacency = {};

		this.graph.nextNodeId = 0;
		this.graph.nextEdgeId = 0;
		this.graph.eventListeners = [];
    console.log(this.graph, init_m, step_m, max_nodes);
    GraphBuilder.BarabasiAlbert(this.graph, init_m, step_m, max_nodes);
    jQuery('#sir_graph').springy({
      graph: this.graph,
      stiffness: 100,
      repulsion: 100,
      minEnergyThreshold: 0.001,
      //damping: 0.6
    });

  }

};


window.onload = function() {
  var sirg = new SirGUIWrap();
  var gui = new dat.GUI();
  var f1 = gui.addFolder('SIR Model');
  f1.add(sirg, 'recovery_rate', 0, 5);
  f1.add(sirg, 'infection_rate', 0, 5);
  f1.add(sirg, 'start');

  var grapg = new GraphGUIWrap(graph);
  var f2 = gui.addFolder('Graph Settings');
  var init_m_controller = f2.add(grapg, 'init_m', 1, 10).step(1);
  var step_m_controller = f2.add(grapg, 'step_m', 1, 10).step(1);
  var nodes_controller = f2.add(grapg, 'max_nodes', 1, 1000).step(10);

  nodes_controller.onFinishChange(function(value) {
    grapg.rebuild(grapg.init_m, grapg.step_m, value);
  });

  init_m_controller.onFinishChange(function(value) {
    grapg.rebuild(value, grapg.step_m, this.max_nodes);
  });

  step_m_controller.onFinishChange(function(value) {
    grapg.rebuild(grapg.init_m, value, grapg.max_nodes);
  });
};
