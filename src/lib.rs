#![deny(warnings)]
extern crate hyper;
extern crate rustc_serialize;
extern crate time;
extern crate libc;

use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::ffi::CStr;
use libc::c_char;

mod aws;
mod kinto;

#[derive(Debug)]
#[derive(RustcEncodable)]
pub struct Update {
    pub last_modified: i64,
    pub id: String
}

#[derive(RustcEncodable)]
#[derive(Debug)]
pub struct Updates {
  pub data: Vec<Update>
}


pub fn generate_update(kinto_url: &str) -> Updates {
    let timespec = time::get_time();
    let mills = timespec.sec + timespec.nsec as i64 / 1000 / 1000;
    let one_day = 60 * 60 * 24;

    // kinto updates
    let kinto_updates = kinto::get_updates(kinto_url);
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


pub fn run(event: String, context: String) -> i32 {
    let decoded_event = Json::from_str(&event).unwrap();
    let event_obj = decoded_event.as_object().unwrap();

    let decoded_context = Json::from_str(&context).unwrap();
    let context_obj = decoded_context.as_object().unwrap();

    println!("Event: {:?}", event_obj);
    println!("Context: {:?}", context_obj);

    // read the options
    let s3_bucket = event_obj.get("s3_bucket").unwrap();
    let s3_filename = event_obj.get("s3_filename").unwrap();
    let kinto_url = event_obj.get("kinto_url").unwrap().as_string().unwrap();

    // generate the update
    let updates = generate_update(&kinto_url);

    // write the update into the s3 bucket
    // let result = json::as_pretty_json(&updates);
    let encoded = json::encode(&updates).unwrap();
    aws::write_s3_file(&s3_bucket.as_string().unwrap(), &s3_filename.as_string().unwrap(), &encoded);
    return 0;
}


#[no_mangle]
pub extern "C" fn handle(c_event: *const c_char,
                         c_context: *const c_char) -> i32 {
    let event = unsafe { CStr::from_ptr(c_event).to_string_lossy()
                         .into_owned() };
    let context = unsafe { CStr::from_ptr(c_context).to_string_lossy()
                           .into_owned() };

    run(event, context)
}
