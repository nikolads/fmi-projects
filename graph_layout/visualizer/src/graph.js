let EventEmitter = require('events');
let Readable = require('stream').Readable;

function Curve() {
    let stream = new Readable({objectMode: true, read: () => {} });
    let prevPoint = null;
    let currPoint = null;

    this.value = function(elapsed) {
        if (prevPoint === null) {
            prevPoint = stream.read();
            if (prevPoint === null) {
                return null;
            }
        }

        if (currPoint === null) {
            currPoint = stream.read();
        }

        while (currPoint !== null && currPoint.time < elapsed) {
            prevPoint = currPoint;
            currPoint = stream.read();
        }

        if (currPoint === null) {
            return [prevPoint.x, prevPoint.y];
        }

        let t = (elapsed - prevPoint.time) / (currPoint.time - prevPoint.time);
        return [(1-t) * prevPoint.x + t * currPoint.x, (1-t) * prevPoint.y + t * currPoint.y];
    }

    let lastPushTime = null;
    this.push = function(data) {
        if (lastPushTime && lastPushTime > data.time) {
            stream.emit('error', new Error('curve data pushed in non-chronological order'));
        }

        lastPushTime = data.time;
        stream.push(data);
    }

    this.eventEmitter = () => stream;
}

/**
 * Creates a new `EventEmitter` wrapping a graph representation.
 */
module.exports.new = function() {
    let graph = {
        vertices: [],
        edges: [],
        curves: {},
    };

    let emitter = new EventEmitter();

    /**
     * Add vertices to the graph.
     *
     * `vertices` is an array of vertex ids to add.
     *
     * Errors:
     * - if a vertex with the same id is already present
     */
    emitter.on('add_vertices', function(vertices) {
        vertices.forEach((v) => {
            if (graph.vertices.includes(v)) {
                // this.emit('error', new Error('vertex already present: ' + v.toString()));
                return;
            }

            graph.vertices.push(v);
        });
    });

    /**
     * Add edges to the graph.
     *
     * `edges` is an array of edges to add, where each edge is a two element array `[v1, v2]` and
     * `v1` and `v2` are vertex ids.
     *
     * Errors:
     * - if either `v1` or `v2` is not a vertex from the graph
     * - if the same edge is already present
     */
    emitter.on('add_edges', function(edges) {
        edges.forEach((edge) => {
            if (!graph.vertices.includes(edge[0])) {
                this.emit('error', new Error('vertex not present: ' + e[0].toString()));
                return;
            }

            if (!graph.vertices.includes(edge[1])) {
                this.emit('error', new Error('vertex not present: ' + e[1].toString()));
                return;
            }

            if (graph.edges.findIndex((el) => el[0] == edge[0] && el[1] == edge[1]) != -1) {
                // this.emit('error', new Error('edge is already present: ' + edge.toString()));
                return;
            }

            graph.edges.push(edge);
        });
    });

    // TODO: docs
    emitter.on('data', function(curves) {
        for (let v in curves) {
            graph.curves[v] = graph.curves[v] || new Curve();
            curves[v].forEach((point) => graph.curves[v].push(point));
        }
    });

    // Empty handler to avoid converting errors to exceptions.
    emitter.on('error', () => {});

    emitter.vertices = () => graph.vertices;
    emitter.edges = () => graph.edges;
    emitter.curve = (v) => graph.curves[v] || null;

    return emitter;
}
