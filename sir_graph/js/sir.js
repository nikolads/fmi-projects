function Sir(graph, infection_prob, recovery_rate) {
    var that = this;
    var intervalId;

    this.step = function() {
        graph.edges.forEach(function(edge) {
            var node1 = edge.source;
            var node2 = edge.target;

            if (node1.data.state == "I" && node2.data.state == "S" && Math.random() < infection_prob) {
                makeInfected(node2);
                that.susceptible--;
                that.infected++;
            }
        });

        graph.nodes.forEach(function(node) {
            if (node.data.state == "I") {
                // node.data.recovery += recovery_rate;
                // if (node.data.recovery > 1.0) {
                //     makeRemoved(node);
                //     that.infected--;
                //     that.recoved++;
                // }

                if (Math.random() < recovery_rate) {
                    makeRemoved(node);
                    that.infected--;
                    that.recoved++;
                }
            }
        });

        console.log(that.susceptible, that.infected, that.recoved);
        graph.notify();

        if (that.susceptible <= 0 || that.infected <= 0) {
            that.stop();
        }
    }

    this.start = function(interval) {
        if (intervalId !== undefined) {
            return;
        }
        calc_distribution();
        intervalId = setInterval(this.step, interval);
    };

    this.stop = function() {
        if (intervalId === undefined) {
            return;
        }
        clearInterval(intervalId);
        intervalId = undefined;
    };

    function calc_distribution() {
        that.susceptible = 0;
        that.infected = 0;
        that.recoved = 0;

        graph.nodes.forEach(function (node) {
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
    node.data.color = "#0000ff";
}

function makeInfected(node) {
    node.data.state = "I";
    node.data.recovery = 0.0;
    node.data.color = "#ff0000";
}

function makeRemoved(node) {
    node.data.state = "R";
    node.data.recovery = undefined;
    node.data.color = "#ff9900";
}
