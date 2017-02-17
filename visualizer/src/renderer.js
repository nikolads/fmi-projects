let binary_search = require('binary-search-bounds');
let d3 = require('d3');

module.exports.init = function(graph) {
    let svg = d3.select('svg');
    let width = svg.attr('width');
    let height = svg.attr('height');

    let gEdge = svg.append('g');
    let gVert = svg.append('g');

    let x = d3.scaleLinear()
        .domain([-30, 30])
        .range([0, width])

    let y = d3.scaleLinear()
        .domain([-30, 30])
        .range([0, height])

    graph.on('add_vertices', function() {
        let sel = gVert.selectAll('circle').data(graph.vertices());

        sel.enter().append('circle')
            .attr('r', 10)
            .attr('fill', (_, i) => '#' + i%10 + i%10 + i%10);

        sel.exit().remove();
    });

    graph.on('add_edges', function() {
        let sel = gEdge.selectAll('line').data(graph.edges());

        sel.enter().append('line')
            .attr('stroke', '#888');

        sel.exit().remove();
    });

    let timer = d3.timer((elapsed) => {
        elapsed = elapsed * 0.001;

        gVert.selectAll('circle').each(function (id) {
            let pos = graph.curve(id).value(elapsed);
            if (pos) {
                d3.select(this)
                    .attr('cx', x(pos[0]))
                    .attr('cy', y(pos[1]));
            }
        });

        gEdge.selectAll('line').each(function ([id1, id2]) {
            let pos1 = graph.curve(id1).value(elapsed);
            let pos2 = graph.curve(id2).value(elapsed);

            if (pos1 && pos2) {
                d3.select(this)
                    .attr('x1', x(pos1[0]))
                    .attr('y1', y(pos1[1]))
                    .attr('x2', x(pos2[0]))
                    .attr('y2', y(pos2[1]));
            }
        });
    });
}
