function Sir(graph, infection_prob, recovery_rate) {
    var that = this;
    var intervalId;

    this.start = function(interval) {
        if (intervalId !== undefined) {
            return;
        }
        intervalId = setInterval(simulation_step, interval);
    };

    this.stop = function() {
        if (intervalId === undefined) {
            return;
        }
        clearInterval(intervalId);
        intervalId = undefined;
    };

    calc_distribution();

    return this;

    function simulation_step() {
        graph.edges.forEach(function(edge) {
            var node1 = edge.source;
            var node2 = edge.target;

            if (node1.data.state == "infected" && node2.data.state == "susceptible" && Math.random() < infection_prob) {
                node2.data.state = "infected";
                node2.data.recovery = 0.0;
                node2.data.color = "#ff0000";
                that.susceptible--;
                that.infected++;
            }
        });

        graph.nodes.forEach(function(node) {
            if (node.data.state == "infected") {
                node.data.recovery += recovery_rate;
                if (node.data.recovery > 1.0) {
                    node.data.state = "recoved";
                    node.data.color = "#ff9900";
                    that.infected--;
                    that.recoved++;
                }
            }
        });

        console.log(that.susceptible, that.infected, that.recoved);

        graph.notify();
    }

    function calc_distribution() {
        console.log(this);

        that.susceptible = 0;
        that.infected = 0;
        that.recoved = 0;

        graph.nodes.forEach(function (node) {
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
}
