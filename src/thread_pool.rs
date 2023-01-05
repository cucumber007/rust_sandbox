use rand::{rngs::ThreadRng, Rng};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub trait ThreadPoolExecutor<Result>: Drop {
    type F: FnOnce() -> Result;
    fn run(&self, code: Self::F);
    fn results(&self) -> mpsc::Iter<'_, Result>;
}

pub struct MyThreadPoolExecutor<T>
where
    T: Send + Sync,
{
    rng: ThreadRng,
    threads: Option<Vec<JoinHandle<()>>>,
    task_tx: Option<Sender<Task<T>>>,
    task_rx: Arc<Mutex<Receiver<Task<T>>>>,
    result_tx: Arc<Mutex<Sender<T>>>,
    result_rx: Option<Receiver<T>>,
}

type Task<T> where T: Send + Sync = Box<dyn FnOnce() -> T + Send + Sync>;

impl<T: 'static> MyThreadPoolExecutor<T>
where
    T: Send + Sync,
{
    pub fn new(threads_num: i32) -> Self {
        let (result_tx, result_rx) = mpsc::channel();
        let (task_tx, task_rx) = mpsc::channel::<Task<T>>();
        let task_rx_link: Arc<Mutex<Receiver<Task<T>>>> = Arc::new(Mutex::new(task_rx));
        let result_tx_link = Arc::new(Mutex::new(result_tx));
        return MyThreadPoolExecutor {
            rng: rand::thread_rng(),
            threads: (0..threads_num)
                .map(|_| {
                    let task_rx = Arc::clone(&task_rx_link);
                    let result_tx = Arc::clone(&result_tx_link);
                    let handle = thread::spawn(move || loop {
                        let message = task_rx.lock().unwrap().recv();
                        match message {
                            Ok(job) => {
                                let result = job();
                                let send_result = result_tx.lock().unwrap().send(result);
                                if let Err(_) = send_result {
                                    break;
                                }
                            },
                            Err(_) => {
                                break;
                            },
                        }
                    });
                    return handle;
                })
                .map(move |x| Some(x))
                .collect(),
            task_tx: Some(task_tx),
            task_rx: task_rx_link,
            result_tx: result_tx_link,
            result_rx: Some(result_rx),
        };
    }
}

impl<T> ThreadPoolExecutor<T> for MyThreadPoolExecutor<T>
where
    T: Send + Sync + 'static,
{
    type F = Box<dyn FnOnce() -> T + Send + Sync>;
    fn run(&self, task: Self::F) {
        self.task_tx.as_ref().unwrap().send(task).unwrap();
    }

    fn results(&self) -> mpsc::Iter<'_, T> {
        self.result_rx.as_ref().unwrap().iter()
    }
}

impl<T> Drop for MyThreadPoolExecutor<T>
where
    T: Send + Sync,
{
    fn drop(&mut self) {
        drop(self.result_rx.take());
        drop(self.task_tx.take());

        if let Some(threads) = self.threads.take() {
            for handle in threads {
                handle.join().unwrap();
            }
        }
    }
}
