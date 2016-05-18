var graph = new Springy.Graph();

//GraphBuilder.BarabasiAlbert(graph, 1, 2, 100, node => {node.data.state = "susceptible"});
GraphBuilder.BarabasiAlbert(graph, 1, 3, 250, function (node) {
  node.data.state = "susceptible";
  node.data.color = "#0000ff";
});


var sir = new Sir(graph, 0.012, 0.04);

jQuery(document).on('random_finished', function(event) {
  sir.start();
});

jQuery(document).on('barabasi_finished', function(event) {
  sir.start();
});


// GraphBuilder.RandomGraph(graph, 500, 1500, function (node) {
//   node.data.state = "susceptible";
//   node.data.color = "#0000ff";
// });
//GraphBuilder.RandomGraph(graph, 250, 230);

graph.nodes[0].data.state = "infected";
graph.nodes[0].data.recovery = 0.0;
graph.nodes[0].data.color = "#ff0000";


var myLineChart;
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

  window.g_springy = springy;
  console.log(springy);

  //sir.start();
  var data = {
    labels: [],
    datasets: [{
        label: '# susceptible',
        fill: false,
        lineTension: 0.1,
        backgroundColor: "rgba(75,192,192,0.4)",
        borderColor: "rgba(75,192,192,1)",
        borderCapStyle: 'butt',
        borderDash: [],
        borderDashOffset: 0.0,
        borderJoinStyle: 'miter',
        pointBorderColor: "rgba(75,192,192,1)",
        pointBackgroundColor: "#fff",
        pointBorderWidth: 1,
        pointHoverRadius: 5,
        pointHoverBackgroundColor: "rgba(75,192,192,1)",
        pointHoverBorderColor: "rgba(220,220,220,1)",
        pointHoverBorderWidth: 2,
        pointRadius: 1,
        pointHitRadius: 10,
        data: [],
      },
      {
        label: '# infected',
        data: [],
      },
    ],
  };

  myLineChart = new Chart($('#sir_graphic'), {
    type: 'line',
    data: data,
  });

  var time = 1;
  jQuery(document).on("sir_simulation_tick", function (event, s, i, r) {
    if (time % 5 != 0)
    {
      time++;
      return;
    }

    myLineChart.data.labels.push(time++);
    myLineChart.data.datasets[0].data.push(s);
    myLineChart.data.datasets[1].data.push(i);
    myLineChart.update();
  })

});
