use std::io::Read;
use std::env;

extern crate hyper;
use hyper::method::Method;

extern crate digitalocean;
use digitalocean::api::baseapi::BaseAPI;
use digitalocean::api::traits::actions::Actions;

fn main() {
    let key = "DIGITALOCEAN_TOKEN";

    match env::var(key) {
        Ok(token) => {
            let mut baseapi = BaseAPI::new(Some(&token));
            let mut json_resp = baseapi.list_all_actions().unwrap();
            println!("{:#?}", json_resp);
        },
        Err(e) => {
            println!("Could not interpret {}: {}", key, e);
        }
    }
    
}