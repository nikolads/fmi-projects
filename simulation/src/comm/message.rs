extern crate json;

use ::force::State;
use self::json::JsonValue;
use self::json::object::Object;
use std::ops::Range;

#[derive(Debug, Clone)]
pub enum OutMessage<T> where
    T: IntoIterator<Item = [f32; 2]> + Clone + Send + 'static,
{
    State(State),
    Data(T, f32),
    Graph(Range<usize>, Vec<(usize, usize)>),
}

impl<T> OutMessage<T> where
    T: IntoIterator<Item = [f32; 2]> + Clone + Send + 'static
{
    pub fn serialize(&self) -> JsonValue {
        match *self {
            OutMessage::State(ref state) => {
                object! {
                    "type" => "state",
                    "state" => state.serialize()
                }
            },
            OutMessage::Data(ref data, time) => {
                object! {
                    "type" => "data",
                    "data" => data.clone().into_iter().enumerate()
                        .map(|(i, pos)| (i, array![ object! {"x" => pos[0], "y" => pos[1], "time" => time } ]))
                        .fold(Object::new(), |mut acc, (i, arr)| {
                            acc.insert(&format!("{}", i), arr);
                            acc
                        })
                }
            },
            OutMessage::Graph(ref vertices, ref edges) => {
                object! {
                    "type" => "graph",
                    "vertices" => vertices.clone().into_iter().collect::<Vec<_>>(),
                    "edges" => edges.iter().map(|&(u, v)| array![u, v]).collect::<Vec<_>>()
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum InMessage {
    New,
    AddVertex,
    AddEdge,
}

impl State {
    fn serialize(&self) -> JsonValue {
        object! {}
    }
}
