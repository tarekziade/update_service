#![deny(warnings)]
extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;
use hyper::{Client};
use rustc_serialize::json;


#[derive(RustcDecodable)]
#[derive(Debug)]
struct KintoUpdate {
    host: String,
    last_modified: usize,
    bucket: String,
    id: String,
    collection: String,
}

#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct KintoUpdates {
  data: Vec<KintoUpdate>
}


fn main() {

    let url = "https://firefox.settings.services.mozilla.com/v1/buckets/monitor/collections/changes/records";
    let client = Client::new();

    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => panic!("Whoops."),
    };

   let mut buf = String::new();
   match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("I give up."),
    };

    let updates: KintoUpdates = json::decode(&buf).unwrap();
    println!("{:?}", updates);
}

