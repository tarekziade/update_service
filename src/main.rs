#![deny(warnings)]
extern crate hyper;
extern crate rustc_serialize;
extern crate time;
extern crate ini;

use ini::Ini;
use std::io::Read;
use std::env;
use std::path::{Path};
use hyper::{Client};
use rustc_serialize::json;


#[derive(Debug)]
#[derive(RustcEncodable)]
struct Update {
    last_modified: i64,
    id: String
}

#[derive(RustcEncodable)]
#[derive(Debug)]
pub struct Updates {
  data: Vec<Update>
}


#[derive(RustcDecodable)]
#[derive(Debug)]
struct KintoUpdate {
    host: String,
    last_modified: i64,
    bucket: String,
    id: String,
    collection: String
}

#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct KintoUpdates {
  data: Vec<KintoUpdate>
}


fn main() {
    let cwd = env::current_dir().unwrap();
    let conf_file = Path::new(&cwd).join("conf.ini");
    let conf;

    match conf_file.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => conf = Ini::load_from_file(s).unwrap(),
    }

    let services = conf.section(Some("services".to_owned())).unwrap();

    let timespec = time::get_time();
    let mills = timespec.sec + timespec.nsec as i64 / 1000 / 1000;
    let one_day = 60 * 60 * 24;
    let url = services.get("kinto_url").unwrap();
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

    let mut general_updates = vec![];

    for update in &updates.data {
       // is this update less than 24h ?
       let delta = mills - (update.last_modified / 1000);
       if delta < one_day {
         println!("Fresh update {:?}", delta);
       } else {
         // we'll skip those later
         println!("Update older than one day {:?}", delta);
       }

       if &*update.collection == "certificates" {
          let id = String::from("onecrl");
          general_updates.push(Update { id: id, last_modified: update.last_modified });
       }
    }

    let result = Updates {data: general_updates};

    println!("{}", json::as_pretty_json(&result));
}

