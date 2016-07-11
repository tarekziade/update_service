#![deny(warnings)]
extern crate hyper;
extern crate rustc_serialize;
extern crate time;

use std::io::Read;
use hyper::{Client};
use rustc_serialize::json;



#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct KintoUpdate {
    pub host: String,
    pub last_modified: i64,
    pub bucket: String,
    pub id: String,
    pub collection: String
}

#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct KintoUpdates {
  pub data: Vec<KintoUpdate>
}


pub fn get_updates(url: &str) -> KintoUpdates {
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


   return json::decode(&buf).unwrap();
}
