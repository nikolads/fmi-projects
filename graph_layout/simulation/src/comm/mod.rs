extern crate json;
extern crate websocket as ws;

use self::message::{InMessage, OutMessage};
use self::multiplex::Multiplexer;
use self::ws::message::{Message as WsMessage, Type};
use self::ws::client::{Sender as WsSender, Receiver as WsReceiver};
use std::io;
use std::net::ToSocketAddrs;
use std::str;
use std::sync::Arc;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};

use self::ws::Sender as WsSenderTrait;
use self::ws::Receiver as WsReceiverTrait;

pub mod message;
pub mod multiplex;

pub struct Server<T> where
    T: IntoIterator<Item = [f32; 2]> + Clone + Send + 'static
{
    thread: JoinHandle<()>,
    msg_queue: Arc<Multiplexer<T>>
}

impl<T> Server<T> where
    T: IntoIterator<Item = [f32; 2]> + Clone + Send + 'static
{
    pub fn start<A>(addr: A) -> Result<(Server<T>, Receiver<InMessage>), io::Error> where
        A: ToSocketAddrs,
    {
        let server = try!(ws::Server::bind(addr));
        println!("Server listening on {}", server.local_addr().unwrap());

        let (in_sender, in_receiver) = mpsc::channel();
        let multiplexer = Arc::new(Multiplexer::<T>::new());

        let handle = {
            let multiplexer = multiplexer.clone();
            try!(thread::Builder::new()
                .name(String::from("server"))
                .spawn(move|| {
                    for connection in server {
                        match connection {
                            Ok(connection) => {
                                let req = connection.read_request().unwrap();
                                println!("Received connection from {}", req.get_reader().peer_addr().unwrap());

                                let res = req.accept();
                                let client = res.send().unwrap();

                                let sender = in_sender.clone();
                                let (cl_sender, cl_receiver) = mpsc::channel();

                                let (ws_sender, ws_receiver) = client.split();

                                let handle = thread::spawn(|| Self::handle_outgoing(ws_sender, cl_receiver));
                                multiplexer.add_listener(handle, cl_sender);

                                thread::spawn(|| Self::handle_incoming(ws_receiver, sender));
                            }
                            Err(err) => panic!(err),
                        }
                    }
                })
            )
        };

        Ok((Server {
            thread: handle,
            msg_queue: multiplexer
        }, in_receiver))
    }

    pub fn msg_queue(&self) -> &Arc<Multiplexer<T>> {
        &self.msg_queue
    }

    fn handle_outgoing(mut client: WsSender<ws::WebSocketStream>, receiver: Receiver<OutMessage<T>>) where
        T: IntoIterator<Item = [f32; 2]> + Clone + Send + 'static
    {
        for msg in receiver {
            let json = msg.serialize();
            if let Err(e) = client.send_message(&ws::Message::text(json.dump())) {
                println!("{}", e);
                return;
            }
        }
    }

    fn handle_incoming(mut client: WsReceiver<ws::WebSocketStream>, sender: Sender<InMessage>) {
        sender.send(InMessage::New).unwrap();

        for msg in client.incoming_messages() {
            let msg: WsMessage = msg.unwrap();
            if let Type::Text = msg.opcode {
                let json = json::parse(str::from_utf8(&msg.payload).unwrap()).unwrap();

                if json["type"] == "addVertex" {
                    sender.send(InMessage::AddVertex).unwrap();
                }
                else if json["type"] == "addEdge" {
                    sender.send(InMessage::AddEdge).unwrap();
                }
                else if json["type"] == "task" {
                    let id = json["id"].as_usize().unwrap();
                    sender.send(InMessage::Task(id)).unwrap();
                }
            }
        }
    }
}
