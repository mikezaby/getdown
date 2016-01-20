extern crate hyper;
extern crate crypto;
extern crate rustc_serialize;

mod downloader;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file = File::open("input.txt").unwrap();
    let file = BufReader::new(file);
    for line in file.lines() {
        downloader::md5(&line.unwrap());
    }
}
