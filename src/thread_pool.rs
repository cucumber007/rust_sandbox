use rand::{Rng, rngs::ThreadRng};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver, Sender};

pub trait ThreadPoolExecutor<Result> {
    type F: FnOnce() -> Result;
    fn run(&self, code: Self::F);
    fn results(&self) -> mpsc::Iter<'_, Result>;
}

pub struct MyThreadPoolExecutor<T> {
    rng: ThreadRng,
    threads: Vec<JoinHandle<()>>,
    rx: Receiver<T>,
    tx: Sender<T>
}

impl <T> MyThreadPoolExecutor<T> {
    pub fn new(threads_num: i32) -> Self {
        let (tx, rx) = mpsc::channel();
        return MyThreadPoolExecutor {
            rng: rand::thread_rng(),
            threads: (0..threads_num).map(|_| thread::spawn(|| {})).collect(),
            rx,
            tx
        };
    }
}

impl<T> ThreadPoolExecutor<T> for MyThreadPoolExecutor<T> {
    type F = Box<dyn FnOnce() -> T>;
    fn run(&self, code: Self::F) {
        let result = code();
        self.tx.send(result).unwrap();
    }

    fn results(&self) -> mpsc::Iter<'_, T> {
        self.rx.iter()
    }
}

