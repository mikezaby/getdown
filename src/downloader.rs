use hyper::client::Client;
use std::io::Read;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use rustc_serialize::hex::ToHex;

pub fn sha1(url: &str) -> String {
    let mut hasher = Sha1::new();
    let mut hash = [0; 20];

    let client = Client::new();
    let response = client.get(url).send().unwrap();

    for byte in response.bytes() {
        hasher.input(&[byte.unwrap()]);
    }

    hasher.result(&mut hash);

    hash.to_hex()
}
