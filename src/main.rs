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

    let task_number = 10;
    for i in 0..task_number {
        let val: f64 = rng.gen();
        println!("Run {} {}", i, val);
        thread_pool.run(Box::new(move || {
            println!("Start {}", val);
            thread::sleep(Duration::from_secs_f64(val));
            return val
        }))
    }

    for (index, res) in thread_pool.results().enumerate() {
        println!("Result {} {}", index, res);
        if index == task_number - 1 {
            break;
        };
    };
}