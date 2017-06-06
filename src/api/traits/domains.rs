use hyper::method::Method;

use serde_json::{Value, Error};

use api::baseapi::BaseAPI;
use api::exceptions::DigitalOceanException;

pub trait Domains {
  /// List all actions
  fn list_all_domains(&mut self) -> Result<Value, DigitalOceanException>;
  /// Create a new Domain
  fn create_domian(&mut self, name: &str, ip_addr: &str) -> Result<Value, DigitalOceanException>;
  /// Retrieve an existing Action
  fn get_domains(&mut self, domain_id: &str) -> Result<Value, DigitalOceanException>;
  // Delete a Domain
  fn delete_domain(&mut self, domain_id: &str) -> Result<Value, DigitalOceanException>;
  /// List all Domain Records
  fn list_domain_records(&mut self, domain_id: &str) -> Result<Value, DigitalOceanException>;
  /// Create a new Domain Record
  fn create_domain_record(&mut self, domain_id: &str /*, rtype: Option<T>, name: Option<T>, data: Option<T>, priority: Option<T>, port: Option<T>, weight: Option<T>*/) -> Result<Value, DigitalOceanException>;
  /// Retrieve an existing Domain Record
  fn get_domain_record(&mut self, domain_id: &str, record_id: &str) -> Result<Value, DigitalOceanException>;
  /// Delete a Domain Record
  fn delete_domain_record(&mut self, domain_id: &str, record_id: &str) -> Result<Value, DigitalOceanException>;
  /// Update a Domain Record
  fn update_domain_record(&mut self, domain_id: &str, record_id: &str, name: &str) -> Result<Value, DigitalOceanException>;
}

impl<'a> Actions for BaseAPI<'a> {
  fn list_all_domains(&mut self) -> Result<Value, DigitalOceanException> {
    self.request(Method::Get, "domains", None)
  }

  fn create_domian(&mut self, name: &str, ip_addr: &str) -> Result<Value, DigitalOceanException> {
    
  }

  fn get_domains(&mut self, domain_id: &str) -> Result<Value, DigitalOceanException> {
    self.request(Method::Get, &format!("domains/{}", domain_id), None)
  }
}