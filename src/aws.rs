extern crate rusoto;
extern crate env_logger;

use self::rusoto::{DefaultCredentialsProvider, Region};
use self::rusoto::s3::S3Helper;


pub fn write_s3_file(bucket_name: &str, filename: &str, content: &str) {
    let _ = env_logger::init();
    let provider = DefaultCredentialsProvider::new().unwrap();
    let s3 = S3Helper::new(provider, Region::UsWest2);
    let response = s3.put_object(bucket_name, filename, content.as_bytes()).unwrap();

    println!("Written in S3: {:?}", response);
}

