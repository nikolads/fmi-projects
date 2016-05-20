function Sir(graph, infection_prob, recovery_rate, sir_tree) {
    var that = this;
    var intervalId;
    that.graph = graph;
    that.tree = sir_tree;
    that.infection_prob = infection_prob;
    that.recovery_rate = recovery_rate;
    that.time = 0;

    this.start = function(interval) {
        calc_distribution();
        if (intervalId !== undefined) {
            return;
        }

        that.time = 0;
        intervalId = setInterval(that.step, interval);

        that.tree.eventListeners[0].layout.clear();
        that.tree.clear();
        jQuery(document).trigger("sir_simulation_start");
    };

    this.stop = function() {
        if (intervalId === undefined) {
            return;
        }
        clearInterval(intervalId);
        intervalId = undefined;
        jQuery(document).trigger("sir_simulation_finished");
    };

    this.step = function() {
        graph.nodes.forEach(function(node) {
            node.data.old_state = node.data.state;
        });

        graph.edges.forEach(function(edge) {
            var node1 = edge.source;
            var node2 = edge.target;

            if (node1.data.old_state == "I" && node2.data.state == "S" && Math.random() < infection_prob) {
                makeInfected(node2);
                that.susceptible--;
                that.infected++;

                node2.data.infected_from = node1.id;
                that.tree.addNode(node1);
                that.tree.addNode(node2);
                that.tree.addEdges([node1.id, node2.id]);
            }
        });

        graph.nodes.forEach(function(node) {
            if (node.data.old_state == "I") {
                node.data.recovery += recovery_rate;
                // if (Math.random() < recovery_rate) {
                if (node.data.recovery > 1.0) {
                    makeRemoved(node);
                    that.infected--;
                    that.recoved++;
                }
            }
        });

        // graph.nodes.forEach(function(node) {
        //     node.data.state = node.data.new_state;
        // });

        that.time++;
        graph.notify();

        // console.log(that.susceptible, that.infected, that.recoved);
        jQuery(document).trigger("sir_simulation_tick", [that.susceptible, that.infected, that.recoved]);

        if (that.susceptible <= 0 || that.infected <= 0) {
            that.stop();
        }
    }

    function calc_distribution() {
        that.susceptible = 0;
        that.infected = 0;
        that.recoved = 0;

        graph.nodes.forEach(function (node) {
            node.data.infected_from = null;
            switch (node.data.state) {
                case "S":
                    that.susceptible++;
                    break;
                case "I":
                    that.infected++;
                    break;
                case "R":
                    that.recovered++;
                    break;
            }
        });
    };

    return this;
}

function makeSusceptible(node) {
    node.data.state = "S";
    node.data.color = "#63D297";
}

function makeInfected(node) {
    node.data.state = "I";
    node.data.recovery = 0.0;
    node.data.color = "#F44336";
}

function makeRemoved(node) {
    node.data.state = "R";
    node.data.recovery = undefined;
    node.data.color = "#FFC107";
}
