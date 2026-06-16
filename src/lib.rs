use std::thread;
use std::sync::{Arc, Mutex, mpsc}; 

type Job = Box<dyn FnOnce() + Send + 'static>; 

enum Message { 
    NewJob(Job), 
    Terminate
}

pub struct ThreadPool { 
    size: usize, 
    workers: Vec<Worker>, 
    sender: mpsc::Sender<Message>
}

impl ThreadPool { 
    pub fn new(size: usize) -> Self { 
        assert!(size >= 1); 
        let (sender, reciever) = mpsc::channel(); 
        let reciever = Arc::new(Mutex::new(reciever)); 

        let mut workers = Vec::with_capacity(size); 
        for i in 0..size { 
            workers.push(Worker::new(i, reciever.clone())); 
        }
        Self { 
            size, 
            workers, 
            sender
        }
    }

    pub fn execute(&mut self, job: Job) { 
        self.sender.send(Message::NewJob(job)).unwrap(); 
    }

    fn drop(&mut self) { 
        self.sender.send(Message::Terminate).unwrap(); 
        for worker in &mut self.workers { 
            println!("Shutting down Worker-{}", worker.id);
            // taking thread out of worker with the help of take() on Option which replaces Some(thread) with None in Worker
            if let Some(thread) = worker.thread.take() { 
                thread.join().unwrap(); 
            }
        }
    }
}


struct Worker { 
    id: usize, 
    thread: Option<thread::JoinHandle<()>>
}

impl Worker { 
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self { 
        let thread = thread::spawn(move || loop {
            let request = reciever.lock().unwrap().recv().unwrap(); 
            match request { 
                Message::NewJob(job) => {
                    println!("Worker-{id} is executing a job!"); 
                    job()
                }, 
                Message::Terminate => {
                    println!("Worker-{id} stopped looking for job requests!");
                    break; 
                }
            }
        }); 

        Self { 
            id, 
            thread: Some(thread)
        }
    }
}