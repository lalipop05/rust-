use std::sync::{Arc, Mutex};
use std::{sync::mpsc, thread};

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender)
        }
    }

    pub fn execute<F>(&self, f: F) 
    where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job;
                {
                    let message = receiver.lock().unwrap().recv();
                    match message {
                        Ok(j) => {
                            println!("Worker {} got a job; executing", id);
                            job = j;
                        },
                        Err(_) => {
                            println!("Worker {} disconnected; shutting down.", id);
                            break;
                        }
                    }
                }
                println!("Worker {} freed", id);
                job();
            }
        });
        Worker { id, thread }
    }
}
