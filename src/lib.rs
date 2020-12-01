use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender, Arc, Mutex};
use std::thread;
type Job = Box<dyn FnOnce() + Send>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl ThreadPool {
    pub fn new(thread_count: usize) -> ThreadPool {
        assert!(thread_count > 0);
        let mut workers = Vec::with_capacity(thread_count);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..thread_count {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }
    pub fn execute(&self, job: Job) {
        self.sender.send(job).unwrap();
    }
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("thread {} running", id);
            job();
        });
        Worker { id, thread }
    }
}
