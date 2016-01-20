use hyper::client::Client;
use std::io::Read;
use crypto::md5::Md5;
use crypto::digest::Digest;
use rustc_serialize::hex::ToHex;

pub fn md5(url: &str) {
    let mut hasher = Md5::new();
    let mut hash = [0; 16];

    let client = Client::new();
    let response = client.get(url).send().unwrap();

    for byte in response.bytes() {
        hasher.input(&[byte.unwrap()]);
    }

    hasher.result(&mut hash);
    println!("{}", hash.to_hex());
}
