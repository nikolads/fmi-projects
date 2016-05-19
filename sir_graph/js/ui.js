var SirGUIWrap = function(gui) {
  this.recovery_rate = 0.5;
  this.infection_rate = 0.5;
  this.start = false;

  var f1 = gui.addFolder('SIR Model');
  f1.add(this, 'recovery_rate', 0, 5);
  f1.add(this, 'infection_rate', 0, 5);
  f1.add(this, 'start');
};

var GraphGUIWrap = function(gui) {
  var that = this;

  this.init_m = 1;
  this.step_m = 1;
  this.max_nodes = 100;

  this.graph = new Springy.Graph();
  this.base_stiffness = 200;
  this.base_repulsion = 100;
  this.minEnergyThreshold = 0.001;

  this.rebuild = function(init_m, step_m, max_nodes) {
    this.graph.nodeSet = {};
    this.graph.nodes = [];
    this.graph.edges = [];
    this.graph.adjacency = {};

    this.graph.nextNodeId = 0;
    this.graph.nextEdgeId = 0;
    this.graph.eventListeners = [];

    console.log(this.graph, init_m, step_m, max_nodes);

    GraphBuilder.BarabasiAlbert(that.graph, init_m, step_m, max_nodes, function(node) {
      node.data.state = "susceptible";
      node.data.color = "#0000ff";
    });

    jQuery('#sir_graph').springy({
      graph: that.graph,
      stiffness: that.base_stiffness / step_m,
      repulsion: that.base_repulsion / step_m,
      minEnergyThreshold: that.minEnergyThreshold,
      //damping: 0.6
    });
  }

  var f2 = gui.addFolder('Graph Settings');
  var init_m_controller = f2.add(this, 'init_m', 1, 10).step(1);
  var step_m_controller = f2.add(this, 'step_m', 1, 10).step(1);
  var nodes_controller = f2.add(this, 'max_nodes', 1, 1000).step(10);

  nodes_controller.onFinishChange(function(value) {
    that.rebuild(that.init_m, that.step_m, value);
  });

  init_m_controller.onFinishChange(function(value) {
    that.rebuild(value, that.step_m, that.max_nodes);
  });

  step_m_controller.onFinishChange(function(value) {
    that.rebuild(that.init_m, value, that.max_nodes);
  });
};

