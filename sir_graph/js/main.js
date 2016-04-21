function getRandomColor() {
    var letters = '0123456789ABCDEF'.split('');
    var color = '#';
    for (var i = 0; i < 6; i++ ) {
        color += letters[Math.floor(Math.random() * 16)];
    }
    return color;
}

function chageColor(g) {
  r_index = Math.floor(Math.random()*graph.edges.length)
  graph.edges[r_index].data.color = getRandomColor();
}

var graph = new Springy.Graph();
// graph.addNodes('Dennis', 'Michael', 'Jessica', 'Timothy', 'Barbara')
// graph.addNodes('Amphitryon', 'Alcmene', 'Iphicles', 'Heracles');
// graph.addEdges(
//   ['Dennis', 'Michael', {color: '#00A0B0', label: 'Foo bar'}],
//   ['Michael', 'Dennis', {color: '#6A4A3C'}],
//   ['Michael', 'Jessica', {color: '#CC333F'}],
//   ['Jessica', 'Barbara', {color: '#EB6841'}],
//   ['Michael', 'Timothy', {color: '#EDC951'}],
//   ['Amphitryon', 'Alcmene', {color: '#7DBE3C'}],
//   ['Alcmene', 'Amphitryon', {color: '#BE7D3C'}],
//   ['Amphitryon', 'Iphicles'],
//   ['Amphitryon', 'Heracles'],
//   ['Barbara', 'Timothy', {color: '#6A4A3C'}]
// );

GraphBuilder.BarabasiAlbert(graph, 2, 50);


jQuery(function(){
  var springy = jQuery('#sir_graph').springy({
    graph: graph,
    stiffness: 10,
    repulsion: 10,
    //minEnergyThreshold: 0.1
  });
  console.log(springy)

  setInterval(function() {
    chageColor(graph);
    graph.notify();
  }, 10);

});
