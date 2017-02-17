module.exports.new = function(graph) {
    let ws = new WebSocket("ws://localhost:44444");

    ws.onmessage = function(event) {
        let msg = JSON.parse(event.data);

        switch (msg.type) {
            case 'data':
                graph.emit('data', msg.data);
                break;

            case 'graph':
                graph.emit('add_vertices', msg.vertices);
                graph.emit('add_edges', msg.edges);
                break;
        }
    };

    return ws;
}
