#![deny(warnings)]
extern crate hyper;
extern crate rustc_serialize;
extern crate time;

use std::io::Read;
use hyper::{Client};
use rustc_serialize::json;


#[derive(RustcDecodable)]
#[derive(Debug)]
struct KintoUpdate {
    host: String,
    last_modified: i64,
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
    let timespec = time::get_time();
    let mills = timespec.sec + timespec.nsec as i64 / 1000 / 1000;
    let one_day = 60 * 60 * 24;
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

    for update in &updates.data {
       // is this update less than 24h ?
       let delta = mills - (update.last_modified / 1000);
       if delta < one_day {
         println!("Fresh update {:?}", delta);
       } else {
         println!("Update older than one day {:?}", delta);
       }
    }
}

