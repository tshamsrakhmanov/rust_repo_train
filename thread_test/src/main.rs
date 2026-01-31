use std::{
    any::Any,
    thread::{self, JoinHandle},
};
pub mod some_func;
use crate::some_func::some_func;
use rand::{self, random_range};
fn main() {
    let count_of_threads = 10;

    let mut bank_of_limits: Vec<u64> = Vec::new();
    let mut bank_of_threads: Vec<JoinHandle<u64>> = Vec::new();

    for _ in 0..count_of_threads {
        let limit1 = random_range(30..50) as u64;
        bank_of_limits.push(limit1);
    }

    for limit in bank_of_limits {
        let h1 = thread::spawn(move || some_func(limit));
        bank_of_threads.push(h1);
    }

    for handle in bank_of_threads {
        let res = handle.join();
        match res {
            Ok(value) => {
                println!("{}", value);
            }
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }

    println!("Main thread finished");
}
