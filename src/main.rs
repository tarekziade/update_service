extern crate update_service;

use update_service::run;


fn main() {
    let event = String::from("event");
    let context = String::from("context");

    run(event, context);
}
