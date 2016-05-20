// global variables
var graph;
var sir;
var myLineChart;

// jQuery(document).on('sir_simulation_finished', function(event, nodes) { ... });

jQuery(document).on("sir_simulation_start", function (event) {
  myLineChart.data.labels = []
  myLineChart.data.datasets[0].data = []
  myLineChart.data.datasets[1].data = []
  myLineChart.data.datasets[2].data = []
  myLineChart.update();
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
});

jQuery(function() {
  graph = new Springy.Graph();
  graph.undirected = true;
  graph.onNodeClick = function (e, node) {
    if(e.shiftKey) {    //Shift+Click
      makeRecovered(node);
    } else {
      makeInfected(node);
    }
  }

  var sir_tree = new Springy.Graph();
  sir_tree.onNodeClick = function (e, node) {
    makeRecovered(node);
    node.data.color = "#8e44ad";
  }
// sir = new Sir(graph, 0.2, 0.08, sir_tree);
  sir = new Sir(graph, 0.012, 0.04, sir_tree);

  var gui = new dat.GUI();
  var sirg = new SirGUIWrap(gui, sir);
  var grapg = new GraphGUIWrap(gui, graph);

  grapg.init_m = 3;
  grapg.step_m = 3;
  grapg.max_nodes = 264;
  grapg.notify();

  grapg.rebuild();

  /*** Init charts ***/
  var springy = jQuery('#sir_graph').springy({
    graph: graph,
    stiffness: 10,
    repulsion: 10,
    minEnergyThreshold: 0.1,
    //damping: 0.6
  });

  var springy_tree = jQuery('#sir_graph_tree').springy({
    graph: sir.tree,
    stiffness: 1000,
    repulsion: 1000,
    minEnergyThreshold: 0.001,
    damping: 0.5
  });

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
