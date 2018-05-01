let graph = require('./graph');
let renderer = require('./renderer');
let ui = require('./ui');
let websocket = require('./websocket');

let emitter = graph.new();
emitter.on('error', console.error);

let ws = websocket.new(emitter);

renderer.init(emitter);
ui.init(emitter, ws);
