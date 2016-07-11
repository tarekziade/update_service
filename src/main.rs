#![deny(warnings)]
extern crate hyper;
extern crate rustc_serialize;
extern crate time;
extern crate ini;

use ini::Ini;
use std::env;
use std::path::{Path};
use rustc_serialize::json;

mod aws;
mod kinto;

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


fn generate_update(conf: &Ini) -> Updates {
    let services = conf.section(Some("services".to_owned())).unwrap();
    let timespec = time::get_time();
    let mills = timespec.sec + timespec.nsec as i64 / 1000 / 1000;
    let one_day = 60 * 60 * 24;

    // kinto updates
    let url = services.get("kinto_url").unwrap();
    let kinto_updates = kinto::get_updates(url);
    let mut general_updates = vec![];

    for update in &kinto_updates.data {
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
    return result;
}


fn main() {
    // read the conf file
    let cwd = env::current_dir().unwrap();
    let conf_file = Path::new(&cwd).join("conf.ini");
    let conf;
    match conf_file.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => conf = Ini::load_from_file(s).unwrap(),
    }

    // generate the update
    let updates = generate_update(&conf);

    // write the update into the s3 bucket
    // let result = json::as_pretty_json(&updates);
    let encoded = json::encode(&updates).unwrap();
    aws::write_s3_file("firefoxpoll", "updates.json", &encoded);
}
