var SirGUIWrap = function(sir) {
    this.sir = sir;
    this.infection_prob = sir.infection_prob;
    this.recovery_rate = sir.recovery_rate;

    this.start = false;
    this.clear = false;
};

var GraphGUIWrap = function(graph) {
    this.init_m = 1;
    this.step_m = 1;
    this.max_nodes = 100;

    this.graph = graph;

    this.rebuild = function(init_m, step_m, max_nodes) {
        this.graph.eventListeners[0].layout.clear();
        this.graph.clear();
        //console.log(this.graph, this.init_m, this.step_m, this.max_nodes);
        GraphBuilder.BarabasiAlbert(this.graph, this.init_m, this.step_m, this.max_nodes,
            function(node) {
                node.data.state = "susceptible";
                node.data.color = "#63D297";
            }
        );
    }

};


window.onload = function() {
    var sirg = new SirGUIWrap(sir);
    var gui = new dat.GUI();
    var f1 = gui.addFolder('SIR Model');
    var rec = f1.add(sirg, 'recovery_rate', 0, 5);
    var inf = f1.add(sirg, 'infection_prob', 0, 1);
    var sir_switch = f1.add(sirg, 'start');
    var sir_clear = f1.add(sirg, 'clear');

    rec.onFinishChange(function(value) {
        sirg.sir.recovery_rate = value;
    });

    inf.onFinishChange(function(value) {
        sirg.sir.infection_prob = value;
    });

    sir_switch.onFinishChange(function(value) {
        if (value)
            sirg.sir.start(100);
        else
            sirg.sir.stop();
    });

    sir_clear.onFinishChange(function(value) {
        sirg.sir.graph.nodes.forEach(function(node) {
          node.data.state = "susceptible";
          node.data.color = "#63D297";
        });
        sirg.clear = false;
        sir_clear.updateDisplay();
    });

    var grapg = new GraphGUIWrap(graph);
    var f2 = gui.addFolder('Graph Settings');
    var init_m_controller = f2.add(grapg, 'init_m', 1, 10).step(1);
    var step_m_controller = f2.add(grapg, 'step_m', 1, 10).step(1);
    var nodes_controller = f2.add(grapg, 'max_nodes', 1, 1000).step(10);

    nodes_controller.onFinishChange(function(value) {
        grapg.rebuild();
    });

    init_m_controller.onFinishChange(function(value) {
        grapg.rebuild();
    });

    step_m_controller.onFinishChange(function(value) {
        grapg.rebuild();
    });
};
