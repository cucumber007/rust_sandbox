#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::env;
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;
use rand::Rng;
use test_rust::thread_pool::MyThreadPoolExecutor;
use test_rust::thread_pool::ThreadPoolExecutor;

fn main() {
    println!("-----------------");

    let mut rng = Box::new(rand::thread_rng());
    let thread_pool = MyThreadPoolExecutor::new(5);

    for i in 0..10 {
        let val: f64 = rng.gen();
        println!("Run {}", val);
        thread_pool.run(Box::new(move || {
            println!("Start {}", val);
            thread::sleep(Duration::from_secs_f64(val));
            return val
        }))
    }

    for res in thread_pool.results() {
        println!("Result {}", res);
    };
}