#[macro_use]
extern crate json;
extern crate rand;

use self::comm::Server;
use self::comm::message::{InMessage, OutMessage};
use self::force::ForceDirected;
use self::rand::distributions::Range;
use std::thread;
use std::time::Duration;

use self::rand::distributions::IndependentSample;

pub mod comm;
pub mod force;

fn main() {
    let (server, msg_queue) = Server::start("127.0.0.1:44444").unwrap();

    let mut force_directed = ForceDirected::new();

    // force_directed.graph_mut().add_vertex();
    // force_directed.graph_mut().add_vertex();
    // force_directed.graph_mut().add_vertex();
    // force_directed.graph_mut().add_vertex();
    // force_directed.graph_mut().add_edge(0, 1);
    // force_directed.graph_mut().add_edge(1, 2);
    // force_directed.graph_mut().add_edge(0, 3);

    force_directed.param.repulsion_mult = 0.5;
    force_directed.param.spring_mult = 0.1;
    force_directed.param.spring_len = 2.0;

    let mut counter = 0_usize;

    loop {
        for msg in msg_queue.try_iter() {
            match msg {
                InMessage::New => server.msg_queue().send(OutMessage::Graph(
                    force_directed.graph().vertices(),
                    force_directed.graph().edges().map(|(&u, v)| (u, v)).filter(|&(u, v)| u < v).collect())),
                InMessage::AddVertex => {
                    force_directed.graph_mut().add_vertex();
                    server.msg_queue().send(OutMessage::Graph(
                        force_directed.graph().vertices(),
                        force_directed.graph().edges().map(|(&u, v)| (u, v)).filter(|&(u, v)| u < v).collect()));
                },
                InMessage::AddEdge => {
                    let mut rng = rand::thread_rng();
                    let verts = Range::new(0, force_directed.graph().vertices().len());
                    let v = verts.ind_sample(&mut rng);
                    let u = verts.ind_sample(&mut rng);

                    if v != u && force_directed.graph().edges().position(|(&e1, e2)| e1 == v && e2 == u).is_none() {
                        force_directed.graph_mut().add_edge(v, u);

                        server.msg_queue().send(OutMessage::Graph(
                            force_directed.graph().vertices(),
                            force_directed.graph().edges().map(|(&u, v)| (u, v)).filter(|&(u, v)| u < v).collect()));
                    }
                },
                InMessage::Task(id) => {
                    counter += 1;
                    server.msg_queue().send(OutMessage::TaskResult(id, counter));
                }
            }
        }

        force_directed.update();
        let (pos, time) = force_directed.pos();

        server.msg_queue().send(OutMessage::Data(pos, time));

        thread::sleep(Duration::from_millis(70));
    }
}
