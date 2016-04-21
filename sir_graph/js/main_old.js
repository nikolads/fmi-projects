function getRandomColor() {
    var letters = '0123456789ABCDEF'.split('');
    var color = '#';
    for (var i = 0; i < 6; i++ ) {
        color += letters[Math.floor(Math.random() * 16)];
    }
    return color;
}

function chageColor(g) {
  graph.edges[Math.round(Math.random()*(graph.edges.length-1))].data.color = getRandomColor();
  graph.nodes[Math.round(Math.random()*(graph.nodes.length-1))].data.color = getRandomColor();
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
  ['Dennis', 'Alcmene', {color: '#00A0B0', label: 'Foo bar'}],
  ['Amphitryon', 'Alcmene', {color: '#7DBE3C'}],
  ['Alcmene', 'Amphitryon', {color: '#BE7D3C'}],
  ['Amphitryon', 'Iphicles'],
  ['Amphitryon', 'Heracles'],
  ['Barbara', 'Timothy', {color: '#6A4A3C'}],
  ['Michael', 'Heracles', {color: '#CC333F'}],

  ['Michael', 'Dennis', {color: '#00A0B0', label: 'Foo bar'}],
  ['Dennis', 'Michael', {color: '#6A4A3C'}],
  ['Jessica', 'Michael', {color: '#CC333F'}],
  ['Barbara', 'Jessica', {color: '#EB6841'}],
  ['Timothy', 'Michael', {color: '#EDC951'}],
  ['Alcmene', 'Dennis', {color: '#00A0B0', label: 'Foo bar'}],
  ['Alcmene', 'Amphitryon', {color: '#7DBE3C'}],
  ['Amphitryon', 'Alcmene', {color: '#BE7D3C'}],
  ['Iphicles', 'Amphitryon'],
  ['Heracles', 'Amphitryon'],
  ['Timothy', 'Barbara', {color: '#6A4A3C'}],
  ['Heracles', 'Michael', {color: '#CC333F'}]
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
