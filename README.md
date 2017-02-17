# Graph Layout

A visualization of a graph using a force directed layout algorithm.

> **Note:** This project is developed as part of the JavaScript course at FMI, Sofia University.

## This is a demo

The project is intended to help with designing a library for easy real time visualization of simulations on the web browser. (TODO: elaborate)

# About the project

## Structure

The project uses a server - client model. It has the following parts:
- `simulation` - the server. It is written in Rust and does all the hard work of calculating the layout.
- `visualization` - the client. It is written in JavaScript and does the rendering of the data.

The two communicate through a websocket, exchanging JSON encoded messages.

## Building

The standart way to build the project is using `cargo` and `npm` respectively

```sh
# build the server
cd simulation
cargo build

# run with
cargo run
```

```sh
# build the client
cd visualization
npm install
node_modules/.bin/browserify src/main.js -o bundle.js

# run by opening `index.html` in a browser
```

## Used technologies for the client
- `d3` - for rendering all the stuff.
- `browserify` - because writing JavaScript code using the Web APIs is a pain.
