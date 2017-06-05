use std::io::Read;
use std::env;

extern crate hyper;
use hyper::method::Method;

extern crate digitalocean;
use digitalocean::api::baseapi::BaseAPI;

fn main() {
    let key = "DIGITALOCEAN_TOKEN";

    match env::var(key) {
        Ok(token) => {
            let mut baseapi = BaseAPI::new(Some("2a05c88b17f75d0dfa497a61185d168046aaa0ccf053bf19376138bb450c77e2"));
            let mut json_resp = baseapi.request(Method::Get, "droplets", None).unwrap();
            println!("{:#?}", json_resp);
        },
        Err(e) => {
            println!("Could not interpret {}: {}", key, e);
        }
    }
    
}