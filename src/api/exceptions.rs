
use hyper::client::response::Response;

#[derive(Debug)]
pub enum DigitalOceanException {
    APIException,
    JSONDecodeError,
    NoTokenProvided,
    ResponseError,
    RequestError,
    ResponseReadError,
}

pub trait ToDigitalOceanException {
    type Res;

    fn into_digitalocean_exception(self) -> Result<Self::Res, DigitalOceanException>;
}

impl ToDigitalOceanException for ::hyper::error::Result<Response> {
    type Res = Response;
    fn into_digitalocean_exception(self) -> Result<Self::Res, DigitalOceanException> {
        match self {
            Ok(resp) => Ok(resp),
            Err(_)   => Err(DigitalOceanException::RequestError)
        }
    }
}