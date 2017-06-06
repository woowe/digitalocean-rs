use hyper::method::Method;

use serde_json::{Value, Error};

use api::baseapi::BaseAPI;
use api::exceptions::DigitalOceanException;

pub trait Actions {
  /// List all actions
  fn list_all_actions(&mut self) -> Result<Value, DigitalOceanException>;
  /// Retrieve an existing Action
  fn get_action(&mut self, action_id: &str) -> Result<Value, DigitalOceanException>;
}

impl<'a> Actions for BaseAPI<'a> {
  fn list_all_actions(&mut self) -> Result<Value, DigitalOceanException> {
    self.request(Method::Get, "actions", None)
  }

  fn get_action(&mut self, action_id: &str) -> Result<Value, DigitalOceanException> {
    self.request(Method::Get, &format!("actions/{}", action_id), None)
  }
}