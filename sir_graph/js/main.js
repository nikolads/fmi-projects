var graph = new Springy.Graph();
graph.undirected = true;
var graph_tree = new Springy.Graph();
var sir = new Sir(graph, 0.2, 0.08, graph_tree);

var myLineChart;

GraphBuilder.BarabasiAlbert(graph, 1, 2, 150, function (node) {
  node.data.state = "susceptible";
  node.data.color = "#63D297";
});

// GraphBuilder.RandomGraph(graph, 151, 601, function (node) {
//   node.data.state = "susceptible";
//   node.data.color = "#63D297";
// });

graph.onNodeClick = function (e, node) {
  console.log(node);
  node.data.state = "infected";
  node.data.recovery = 0.0;
  node.data.color = "#F44336";
  console.log(e);
  if(e.shiftKey) {
    //Ctrl+Click
    node.data.state = "recoved";
    node.data.color = "#FFC107";
  }

}

graph_tree.onNodeClick = function (e, node) {
  console.log(node);
  node.data.color = "#8e44ad";
}

jQuery(document).on('random_finished barabasi_finished', function(event) {
  console.log('GraphBuilder finished');
});

jQuery(document).on('sir_simulation_finished', function(event, nodes) {
  //GraphBuilder.Tree(graph, graph_tree, graph.nodes[0].id);
});

jQuery(document).on("sir_simulation_start", function (event) {
  graph_tree.eventListeners[0].layout.clear();
  graph_tree.clear();
  myLineChart.data.labels = []
  myLineChart.data.datasets[0].data = []
  myLineChart.data.datasets[1].data = []
  myLineChart.data.datasets[2].data = []
  myLineChart.update();
  console.log("sir_start");
})

jQuery(document).on("sir_simulation_tick", function (event, s, i, r) {
  if (sir.time > 1 && sir.time % 3 != 0)
    return;

  myLineChart.data.labels.push(sir.time);
  myLineChart.data.datasets[0].data.push(s);
  myLineChart.data.datasets[1].data.push(i);
  myLineChart.data.datasets[2].data.push(r);
  myLineChart.update();
})

jQuery(function(){

   document.getElementById("sir_graph").width = window.innerWidth;
   document.getElementById("sir_graph").height = window.innerHeight;

  var springy = jQuery('#sir_graph').springy({
    graph: graph,
    stiffness: 10,
    repulsion: 10,
    minEnergyThreshold: 0.01,
    //damping: 0.6
  });

  var springy_tree = jQuery('#sir_graph_tree').springy({
    graph: graph_tree,
    stiffness: 200,
    repulsion: 200,
    minEnergyThreshold: 0.001,
    damping: 0.5
  });

  window.g_springy = springy;
  console.log(springy);

  //sir.start();
  var data = {
    labels: [],
    datasets: [{
        scaleFontColor: '#fff',
        label: '# susceptible',
        lineTension: 0,
        backgroundColor: "rgba(99, 210, 151, 0.3)",
        borderColor: "#63D297",
        pointBackgroundColor: "#63D297",
        data: [],
      },
      {
        label: '# infected',
        lineTension: 0,
        backgroundColor: "rgba(244,67,54,0.5)",
        borderColor: "#F44336",
        pointBackgroundColor: "#F44336",
        data: [],
      },
      {
        label: '# recoverd',
        lineTension: 0,
        backgroundColor: "rgba(255,193,7,0.5)",
        borderColor: "#FFC107",
        pointBackgroundColor: "#FFC107",
        data: [],
      }
    ],
  };
  Chart.defaults.global.defaultFontColor = "#fff";

  myLineChart = new Chart($('#sir_graphic'), {
    type: 'line',
    data: data,
    options: {
      legend : {
        labels : {
          fontSize: 16,
        }
      }
    }
  });

  $("#sir_graph_tree").attr("width",$("#chart").innerWidth());
  $("#sir_graph_tree").attr("height",$("#chart").innerHeight());

});
