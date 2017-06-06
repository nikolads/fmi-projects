use std::boxed::FnBox;
use std::io;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, Builder, JoinHandle};

enum Msg {
    Exec(Box<FnBox() + Send + 'static>),
    Exit
}

/// Thread pool
pub struct Pool {
    threads: Vec<(JoinHandle<()>, Sender<Msg>)>
}

impl Pool {
    pub fn new(nthreads: usize) -> Result<Pool, io::Error> {
        let mut threads = Vec::new();

        for i in 0..nthreads {
            let (sender, receiver) = mpsc::channel();

            let thread_main = move || {
                loop {
                    for msg in receiver.try_iter() {
                        match msg {
                            Msg::Exec(func) => func(),
                            Msg::Exit => return,
                        }
                    }

                    thread::park();
                }
            };

            let handle = Builder::new()
                .name(format!("thread #{}", i))
                .spawn(thread_main)?;


            threads.push((handle, sender));
        };

        Ok(Pool { threads })
    }

    /// Return an iterator over the threads in this pool.
    pub fn threads<'a>(&'a self) -> impl Iterator<Item = Handle<'a>> {
        self.threads.iter().map(|handle| Handle { inner: handle })
    }

    /// Block and wait all threads in the pool to finish their scheduled tasks.
    pub fn join(self) -> thread::Result<()> {
        for &(ref handle, ref sender) in &self.threads {
            let _ = sender.send(Msg::Exit);
            handle.thread().unpark();
        }

        for (handle, _) in self.threads {
            handle.join()?;
        }

        Ok(())
    }
}

/// Handle to a thread in a thread pool.
///
/// Can be used to execute a function on this thread.
pub struct Handle<'a> {
    inner: &'a (JoinHandle<()>, Sender<Msg>),
}

impl<'a> Handle<'a> {
    /// Schedule a function to be executed on the thread.
    pub fn execute<F> (&self, func: F) where
        F: FnOnce() + Send + 'static
    {
        let &(ref handle, ref sender) = self.inner;

        // if `send` fails then the thread must have paniced
        sender.send(Msg::Exec(Box::new(func))).unwrap();
        handle.thread().unpark();
    }
}
