extern crate rand;
extern crate vecmath;

use self::graph::Graph;
use self::rand::distributions::Range;
use self::vecmath::{vec2_add, vec2_len, vec2_scale, vec2_square_len, vec2_normalized_sub, vec2_sub};
use std::default::Default;
use std::sync::Arc;

use self::rand::distributions::IndependentSample;

type Vec2 = self::vecmath::Vector2<f32>;

pub mod graph;

pub struct ForceDirected {
    graph: Graph,
    pub param: State,

    accel: Vec<Vec2>,
    veloc: Vec<Vec2>,
    pos: Arc<Vec<Vec2>>,
    time: f32,
}

impl ForceDirected {
    pub fn new() -> ForceDirected {
        ForceDirected {
            graph: Graph::new(),
            param: State::default(),
            accel: Vec::new(),
            veloc: Vec::new(),
            pos: Arc::new(Vec::new()),
            time: 0.0,
        }
    }

    pub fn update(&mut self) {
        let mut accel = self.accel.clone();
        accel.resize(self.graph.vertices().len(), [0.0, 0.0]);

        let mut veloc = self.veloc.clone();
        veloc.resize(self.graph.vertices().len(), [0.0, 0.0]);

        let mut pos = (*self.pos).clone();
        pos.extend((self.pos.len()..self.graph.vertices().len()).into_iter()
            .map(|_| [Range::new(-5.0, 5.0).ind_sample(&mut rand::thread_rng()), Range::new(-5.0, 5.0).ind_sample(&mut rand::thread_rng())]));

        for v in self.graph.vertices() {
            for u in self.graph.vertices() {
                if v != u {
                    accel[v] = vec2_add(accel[v], self.repulsion_force(pos[v], pos[u]));
                }
            }
        }

        for (&v, u) in self.graph.edges() {
            accel[v] = vec2_add(accel[v], self.spring_force(pos[v], pos[u]));
        }

        // add drag?
        for v in self.graph.vertices() {
            accel[v] = vec2_sub(accel[v], vec2_scale(veloc[v], vec2_square_len(veloc[v]) * 0.01));
        }

        for v in self.graph.vertices() {
            veloc[v] = vec2_add(veloc[v], vec2_scale(vec2_add(*self.accel.get(v).unwrap_or(&[0.0, 0.0]), accel[v]), 0.5 * self.param.dtime));
        }

        for v in self.graph.vertices() {
            pos[v] = vec2_add(pos[v], vec2_scale(vec2_add(*self.veloc.get(v).unwrap_or(&[0.0, 0.0]), veloc[v]), 0.5 * self.param.dtime));
        }

        self.accel = accel;
        self.veloc = veloc;
        self.pos = Arc::new(pos);
        self.time += self.param.dtime;
    }

    pub fn pos(&self) -> (PosIter, f32) {
        (PosIter::new(self.pos.clone()), self.time)
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }

    fn repulsion_force(&self, a: Vec2, b: Vec2) -> Vec2 {
        let dir = vec2_normalized_sub(a, b);
        let mag = f32::min(self.param.repulsion_mult * 1.0 / vec2_square_len(dir), 2.0);
        vec2_scale(dir, mag)
    }

    fn spring_force(&self, a: Vec2, b: Vec2) -> Vec2 {
        let dir = vec2_normalized_sub(b, a);
        let mag = f32::min(self.param.spring_mult * (vec2_len(vec2_sub(b, a)) - self.param.spring_len), 2.0);
        vec2_scale(dir, mag)
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub repulsion_mult: f32,
    pub spring_mult: f32,
    pub spring_len: f32,
    pub dtime: f32,
}

impl Default for State {
    fn default() -> State {
        State {
            repulsion_mult: 1.0,
            spring_mult: 1.0,
            spring_len: 1.0,
            dtime: 0.01,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PosIter {
    arc: Arc<Vec<Vec2>>,
    i: usize,
}

impl PosIter {
    fn new(arc: Arc<Vec<Vec2>>) -> PosIter {
        PosIter {
            arc: arc,
            i: 0,
        }
    }
}

impl Iterator for PosIter {
    type Item = Vec2;

    fn next(&mut self) -> Option<Vec2> {
        let curr_i = self.i;
        self.i += 1;

        self.arc.get(curr_i).cloned()
    }
}
