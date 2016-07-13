extern crate update_service;
extern crate rustc_serialize;

use rustc_serialize::json;
use update_service::run;

#[derive(RustcDecodable, RustcEncodable)]
struct Event {
  s3_bucket: String,
  s3_filename: String,
  kinto_url: String
}

fn main() {

    let event = Event {s3_bucket: "firefoxpoll".to_string(),
                       s3_filename: "updates.json".to_string(),
                       kinto_url: "https://firefox.settings.services.mozilla.com/v1/buckets/monitor/collections/changes/records".to_string()};

    let encoded_event = json::encode(&event).unwrap();

    let context = String::from("{}");
    run(encoded_event, context);
}
