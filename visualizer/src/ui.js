module.exports.init = function(graph, ws) {
    document.getElementById('add-vertex').addEventListener('click', function(evt) {
        ws.send(JSON.stringify({
            type: 'addVertex',
        }));
    });

    document.getElementById('add-edge').addEventListener('click', function(evt) {
        ws.send(JSON.stringify({
            type: 'addEdge'
        }));
    });

    document.getElementById('task').addEventListener('click', function(evt) {
        console.log(graph.edges());

        ws.send(JSON.stringify({
            type: 'task',
            id: 0,
        }));
    });
}
