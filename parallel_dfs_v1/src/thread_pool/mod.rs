extern crate crossbeam;

use self::crossbeam::sync::chase_lev::{self, Steal, Stealer, Worker};

use std::boxed::FnBox;
use std::io;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::Builder;

enum Msg {
    Run(Box<FnBox() + Send + 'static>),
    Close
}

/// Thread pool
pub struct Pool {
    threads: usize,
    condvar: Arc<CondvarPair>,
    queue: Worker<Msg>,
}

impl Pool {
    pub fn new(nthreads: usize) -> Result<Pool, io::Error> {
        let mut threads = Vec::new();
        let (worker, stealer) = chase_lev::deque();
        let condvar = Arc::new(CondvarPair::new());

        for i in 0..nthreads {
            let stealer = stealer.clone();
            let condvar = condvar.clone();

            let handle = Builder::new()
                .name(format!("thread #{}", i))
                .spawn(move || { thread_main(stealer, condvar) })?;

            threads.push(handle);
        };

        Ok(Pool {
            threads: nthreads,
            condvar: condvar,
            queue: worker,
        })
    }

    pub fn spawn<F>(&mut self, task: F) where
        F: FnOnce() + Send + 'static
    {
        self.queue.push(Msg::Run(Box::new(task)));
        self.condvar.var.notify_one();
    }

    /// Block until all tasks are done
    pub fn wait(&self) {
        unimplemented!();
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        for _ in 0..self.threads {
            self.queue.push(Msg::Close);
        }
        self.condvar.var.notify_all();
    }
}

struct CondvarPair {
    mutex: Mutex<()>,
    var: Condvar,
}

impl CondvarPair {
    fn new() -> Self {
        CondvarPair {
            mutex: Mutex::new(()),
            var: Condvar::new(),
        }
    }
}

fn thread_main(stealer: Stealer<Msg>, condvar: Arc<CondvarPair>) {
    loop {
        match stealer.steal() {
            Steal::Empty => {
                let lock = condvar.mutex.lock().unwrap();
                drop(condvar.var.wait(lock).unwrap());
            },
            Steal::Abort => {
                // exponential_backoff()
            },
            Steal::Data(Msg::Run(func)) => {
                func();
            },
            Steal::Data(Msg::Close) => {
                // wake pool waiters
                break;
            }
        }
    }
}
