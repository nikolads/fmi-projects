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
}
