extern crate update_service;
extern crate ini;

use update_service::generate_update;
use std::env;
use std::path::{Path};
use ini::Ini;


#[test]
fn test_generate_update() {
    let cwd = env::current_dir().unwrap();
    let conf_file = Path::new(&cwd).join("conf.ini");
    let conf;
    match conf_file.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => conf = Ini::load_from_file(s).unwrap(),
    }

    // generate the update
    let updates = generate_update(&conf);

    // todo: mock the HTTP call
    assert!(updates.data.len() == 1);

}
