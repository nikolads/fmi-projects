var SirGUIWrap = function(gui, sir) {
    var that = this;
    this.sir = sir;
    this.recovery_rate = sir.recovery_rate;
    this.infection_prob = sir.infection_prob;

    this.start = false;
    this.clear = false;

    this.notify = function() {
        rec_rate_controller.updateDisplay();
        inf_rate_controller.updateDisplay();
        sir_switch.updateDisplay();
        sir_clear.updateDisplay();
    }

    var f1 = gui.addFolder('SIR Model');
    var rec_rate_controller = f1.add(this, 'recovery_rate', 0, 1);
    var inf_rate_controller = f1.add(this, 'infection_prob', 0, 1);
    var sir_switch = f1.add(this, 'start');
    var sir_clear = f1.add(this, 'clear');

    rec_rate_controller.onFinishChange(function(value) {
        that.sir.recovery_rate = value;
    });

    inf_rate_controller.onFinishChange(function(value) {
        that.sir.infection_prob = value;
    });

    sir_switch.onFinishChange(function(value) {
        if (value) {
            that.sir.start(100);
        } else {
            that.sir.stop();
        }
    });

    sir_clear.onFinishChange(function(value) {
        that.sir.graph.nodes.forEach(makeSusceptible);
        that.clear = false;
        sir_clear.updateDisplay();
    });
};

var GraphGUIWrap = function(gui, graph) {
    var that = this;

    this.init_m = 1;
    this.step_m = 1;
    this.max_nodes = 100;

    this.graph = graph;

    this.rebuild = function() {
        if (this.graph.eventListeners[0]) {
            this.graph.eventListeners[0].layout.clear();
            this.graph.clear();
        }
        GraphBuilder.BarabasiAlbert(that.graph, that.init_m, that.step_m, that.max_nodes, makeSusceptible);
    }

    this.notify = function() {
        init_m_controller.updateDisplay();
        step_m_controller.updateDisplay();
        nodes_controller.updateDisplay();
    }

    var f2 = gui.addFolder('Graph Settings');
    var init_m_controller = f2.add(this, 'init_m', 1, 10).step(1);
    var step_m_controller = f2.add(this, 'step_m', 1, 10).step(1);
    var nodes_controller = f2.add(this, 'max_nodes', 1, 1000).step(10);

    nodes_controller.onFinishChange(function(value) {
        that.rebuild();
    });

    init_m_controller.onFinishChange(function(value) {
        that.rebuild();
    });

    step_m_controller.onFinishChange(function(value) {
        that.rebuild();
    });
};
