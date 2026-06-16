use std::thread;
use std::sync::{Arc, Mutex, mpsc}; 

type Job = Box<dyn FnOnce() + Send + 'static>; 

pub struct ThreadPool { 
    size: usize, 
    workers: Vec<Worker>, 
    sender: mpsc::Sender<Job>
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
        self.sender.send(job).unwrap(); 
    }
}


struct Worker { 
    id: usize, 
    thread: thread::JoinHandle<()>
}

impl Worker { 
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self { 
        let thread = thread::spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap(); 
            println!("Worker-{id} is executing a job!"); 
            job();
        }); 

        Self { 
            id, 
            thread
        }
    }
}