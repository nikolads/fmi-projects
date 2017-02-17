use super::message::OutMessage;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::thread::JoinHandle;

use std::clone::Clone;

pub struct Multiplexer<T> where
    T: IntoIterator<Item = [f32; 2]> + Clone + Send + 'static
{
    senders: Mutex<Vec<(Sender<OutMessage<T>>, JoinHandle<()>)>>,
}

impl<T: IntoIterator<Item = [f32; 2]> + Clone + Send + 'static> Multiplexer<T> {
    pub fn new() -> Multiplexer<T> {
        Multiplexer {
            senders: Mutex::new(Vec::new()),
        }
    }

    pub fn send(&self, data: OutMessage<T>){
        let mut senders = self.senders.lock().unwrap();
        senders.retain(|&(ref s, ref l)| match s.send(data.clone()) {
            Ok(_) => {
                l.thread().unpark();
                true
            },
            Err(e) => {
                println!("{}", e);
                false
            }
        });
    }

    pub fn add_listener(&self, thread: JoinHandle<()>, sender: Sender<OutMessage<T>>) {
        let mut senders = self.senders.lock().unwrap();
        senders.push((sender, thread));
    }
}
