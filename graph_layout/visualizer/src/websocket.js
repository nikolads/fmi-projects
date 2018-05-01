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

            case 'taskResult':
                console.log(msg.id + " => " + msg.res);
                let parent = msg.id;

                graph.edges().filter(e => e[0] === parent).forEach(e => {
                    ws.send(JSON.stringify({
                        type: 'task',
                        id: e[1],
                    }));
                })

                break;
        }
    };

    return ws;
}
