extern crate threadpool;
extern crate hyper;
extern crate crypto;
extern crate rustc_serialize;

mod downloader;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use threadpool::ThreadPool;
use std::sync::mpsc;

fn main() {
    let file = File::open("input.txt").unwrap();
    let file = BufReader::new(file);

    let thread_pool = ThreadPool::new(100);
    let (sender, receiver) = mpsc::channel();

    for line in file.lines() {
        let sender = sender.clone();

        thread_pool.execute(move || {
            sender.send(downloader::sha1(&line.unwrap())).unwrap();
        });
    }

    while thread_pool.active_count() > 0 {
        println!("{}", receiver.recv().unwrap());
    }
}
