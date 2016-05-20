function Sir(graph, infection_prob, recovery_rate, sir_tree) {
    var that = this;
    var intervalId;
    that.graph = graph;
    that.sir_tree = sir_tree;
    that.infection_prob = infection_prob;
    that.recovery_rate = recovery_rate;
    that.time = 0;

    this.start = function(interval) {
        console.log('start sir simulation');
        jQuery(document).trigger("sir_simulation_start");
        if (intervalId !== undefined) {
            this.stop();
        }
        calc_distribution();
        that.time = 0;
        intervalId = setInterval(simulation_step, interval);
    };

    this.stop = function() {
        if (intervalId === undefined) {
            return;
        }
        clearInterval(intervalId);
        intervalId = undefined;
    };

    function simulation_step() {

        graph.nodes.forEach(function(node) {
            node.data.new_state = node.data.state;
        });

        graph.edges.forEach(function(edge) {
            var node1 = edge.source;
            var node2 = edge.target;

            if (node1.data.state == "susceptible" && node1.data.state == "infected") {
                tmp = node1; node1 = node2; node2 = tmp;
            }

            if (node1.data.state == "infected" && node2.data.state == "susceptible" && node2.data.new_state == "susceptible" && Math.random() < that.infection_prob) {
                node2.data.new_state = "infected";
                node2.data.recovery = 0.0;
                node2.data.color = "#F44336";
                that.susceptible--;
                that.infected++;

                node2.data.infected_from = node1.id;
                that.sir_tree.addNode(node1);
                that.sir_tree.addNode(node2);
                that.sir_tree.addEdges([node1.id, node2.id]);
            }
        });

        graph.nodes.forEach(function(node) {
            if (node.data.state == "infected") {
                if (Math.random() < that.recovery_rate) {
                    node.data.new_state = "recoved";
                    node.data.color = "#FFC107";
                    that.infected--;
                    that.recoved++;
                }
            }
        });

        graph.nodes.forEach(function(node) {
            node.data.state = node.data.new_state;
        });

        that.time++;
        jQuery(document).trigger("sir_simulation_tick", [that.susceptible, that.infected, that.recoved]);
        graph.notify();
        if (that.infected <= 0) {
            clearInterval(intervalId);
            jQuery(document).trigger("sir_simulation_finished");
        }
    }

    function calc_distribution() {
        that.susceptible = 0;
        that.infected = 0;
        that.recoved = 0;

        graph.nodes.forEach(function (node) {
            node.data.infected_from = null;
            switch (node.data.state) {
                case "susceptible":
                    that.susceptible++;
                    break;
                case "infected":
                    that.infected++;
                    break;
                case "recovered":
                    that.recovered++;
                    break;
            }
        });
    };

    return this;
}
