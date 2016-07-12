extern crate update_service;
use update_service::generate_update;


#[test]
fn test_generate_update() {
    // generate the update
    let kinto_url = String::from("https://firefox.settings.services.mozilla.com/v1/buckets/monitor/collections/changes/records");
    let updates = generate_update(&kinto_url);

    // todo: mock the HTTP call
    assert!(updates.data.len() == 1);

}
