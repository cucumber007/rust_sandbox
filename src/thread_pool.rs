use rand::{rngs::ThreadRng, Rng};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub trait ThreadPoolExecutor<Result> {
    type F: FnOnce() -> Result;
    fn run(&self, code: Self::F);
    fn results(&self) -> mpsc::Iter<'_, Result>;
}

pub struct MyThreadPoolExecutor<T> where T: Send + Sync {
    rng: ThreadRng,
    threads: Vec<JoinHandle<()>>,
    task_tx: Sender<Box<dyn FnOnce() -> T + Send + Sync>>,
    task_rx: Arc<Mutex<Receiver<Box<dyn FnOnce() -> T + Send + Sync>>>>,
    result_tx: Arc<Mutex<Sender<T>>>,
    result_rx: Receiver<T>
}

impl<T: 'static> MyThreadPoolExecutor<T> where T: Send + Sync {
    pub fn new(threads_num: i32) -> Self {
        let (result_tx, result_rx) = mpsc::channel();
        let (
            task_tx, 
            task_rx
        ) = mpsc::channel();
        let task_rx_link: Arc<Mutex<Receiver<Box<dyn FnOnce() -> T + Send + Sync>>>> = 
            Arc::new(Mutex::new(task_rx));
        let result_tx_link =
            Arc::new(Mutex::new(result_tx));
        return MyThreadPoolExecutor {
            rng: rand::thread_rng(),
            threads: (0..threads_num)
                .map(|_| {
                    let task_rx = Arc::clone(&task_rx_link);
                    let result_tx = Arc::clone(&result_tx_link);
                    thread::spawn(move || {
                        loop {
                            let task: Box<dyn FnOnce() -> T + Send + Sync> = task_rx.lock().unwrap().recv().unwrap();
                            let result = task();
                            result_tx.lock().unwrap().send(result).unwrap();
                        }
                    })
                })
                .collect(),
                task_tx,
                task_rx: task_rx_link,
                result_tx: result_tx_link,
                result_rx
        };
    }
}

impl<T> ThreadPoolExecutor<T> for MyThreadPoolExecutor<T>
where
    T: Send + Sync + 'static,
{
    type F = Box<dyn FnOnce() -> T + Send + Sync>;
    fn run(&self, task: Self::F) {
        self.task_tx.send(Box::new(task)).unwrap();
    }

    fn results(&self) -> mpsc::Iter<'_, T> {
        self.result_rx.iter()
    }
}
