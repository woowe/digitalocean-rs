use std::net::SocketAddr;
use std::str::FromStr;
use std::io::Read;

use hyper::net::HttpsConnector;
use hyper::header::{Headers, ContentType, Authorization, Bearer};
use hyper::method::Method;
use hyper::client::Client;
use hyper::client::response::Response;

use hyper_native_tls::NativeTlsClient;

use serde_json::{Value, Error};

use api::exceptions::{DigitalOceanException, ToDigitalOceanException};

#[derive(Debug)]
pub struct BaseAPI<'a> {
    endpoint: &'a str,
    token: Option<&'a str>,
    headers: Headers,
    client: Client
}

impl<'a> BaseAPI<'a> {
    pub fn new(token: Option<&'a str>) -> BaseAPI<'a> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);

        BaseAPI {
            endpoint: "https://api.digitalocean.com/v2/",
            token: token,
            headers: Headers::new(),
            client: Client::with_connector(connector)
        }
    }

    fn __set_content_type(&mut self, ctype: ContentType) {
        self.headers.set(ctype);
    }

    fn __set_authorization(&mut self) -> Result<(), DigitalOceanException> {
        if let Some(t) = self.token {
            self.headers.set(Authorization(
                Bearer {
                    token: t.to_owned()
                }
            ));

            Ok(())
        } else {
            Err(DigitalOceanException::NoTokenProvided)
        }
    }

    fn __make_url(&self, path: &str, params: Option<&str>) -> String {
        let _params = match params {
            Some(p) => p,
            None    => ""
        };

        format!("{}{}?{}", self.endpoint, path, _params)
    }

    fn __get(&self, path: &str, params: Option<&str>) -> Result<Response, DigitalOceanException> {
        let url = self.__make_url(path, params);
        let headers = self.headers.clone();
        self.client.get(&url)
            .headers(headers)
            .send()
            .into_digitalocean_exception()
    }

    fn __post(&self, path: &str, params: Option<&str>) -> Result<Response, DigitalOceanException> {
        let url = self.__make_url(path, params);
        let headers = self.headers.clone();
        self.client.post(&url)
            .headers(headers)
            .send()
            .into_digitalocean_exception()
    }

    fn __put(&self, path: &str, params: Option<&str>) -> Result<Response, DigitalOceanException> {
        let url = self.__make_url(path, params);
        let headers = self.headers.clone();
        self.client.put(&url)
            .headers(headers)
            .send()
            .into_digitalocean_exception()
    }

    fn __delete(&self, path: &str, params: Option<&str>) -> Result<Response, DigitalOceanException> {
        let url = self.__make_url(path, params);
        let headers = self.headers.clone();
        self.client.delete(&url)
            .headers(headers)
            .send()
            .into_digitalocean_exception()
    }

    fn __head(&self, path: &str, params: Option<&str>) -> Result<Response, DigitalOceanException> {
        let url = self.__make_url(path, params);
        let headers = self.headers.clone();
        self.client.head(&url)
            .headers(headers)
            .send()
            .into_digitalocean_exception()
    }

    fn __request(&mut self, method: Method, path: &str, params: Option<&str>) -> Result<Response, DigitalOceanException> {
        self.__set_authorization()?;

        match method {
            Method::Get => self.__get(path, params),
            Method::Post => self.__post(path, params),
            Method::Put => self.__put(path, params),
            Method::Delete => self.__delete(path, params),
            Method::Head => self.__head(path, params),
            _ => self.__get(path, params)
        }
    }

    pub fn request(&mut self, method: Method, path: &str, params: Option<&str>) -> Result<Value, DigitalOceanException> {
        let mut resp = self.__request(method, path, params)?;
        let mut body = Vec::new();
        match resp.read_to_end(&mut body) {
            Ok(_) => {},
            Err(_) => return Err(DigitalOceanException::ResponseReadError)
        };

        let mut body_str = String::from_utf8_lossy(&body).into_owned();
        match ::serde_json::from_str(&body_str) {
            Ok(v) => Ok(v),
            Err(_)  => Err(DigitalOceanException::JSONDecodeError)
        }
    }
}