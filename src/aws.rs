extern crate rusoto;
extern crate env_logger;

use self::rusoto::{ProfileProvider, Region};
use self::rusoto::s3::S3Helper;


pub fn write_s3_file() {
    let _ = env_logger::init();

    let s3 = S3Helper::new(ProfileProvider::new().unwrap(), Region::UsWest2);
    let response = s3.list_buckets();
    println!("Got list of buckets: {:?}", response);

}

