function getRandomColor() {
    var letters = '0123456789ABCDEF'.split('');
    var color = '#';
    for (var i = 0; i < 6; i++ ) {
        color += letters[Math.floor(Math.random() * 16)];
    }
    return color;
}

function chageColor(g) {
  graph.edges[Math.round(Math.random()*10)].data.color = getRandomColor();
}

var graph = new Springy.Graph();
graph.addNodes('Dennis', 'Michael', 'Jessica', 'Timothy', 'Barbara')
graph.addNodes('Amphitryon', 'Alcmene', 'Iphicles', 'Heracles');
graph.addEdges(
  ['Dennis', 'Michael', {color: '#00A0B0', label: 'Foo bar'}],
  ['Michael', 'Dennis', {color: '#6A4A3C'}],
  ['Michael', 'Jessica', {color: '#CC333F'}],
  ['Jessica', 'Barbara', {color: '#EB6841'}],
  ['Michael', 'Timothy', {color: '#EDC951'}],
  ['Amphitryon', 'Alcmene', {color: '#7DBE3C'}],
  ['Alcmene', 'Amphitryon', {color: '#BE7D3C'}],
  ['Amphitryon', 'Iphicles'],
  ['Amphitryon', 'Heracles'],
  ['Barbara', 'Timothy', {color: '#6A4A3C'}]
);

jQuery(function(){
  var springy = jQuery('#sir_graph').springy({
    graph: graph
  });

  setInterval(function() {
    chageColor(graph);
    graph.notify();
  }, 10);

});
