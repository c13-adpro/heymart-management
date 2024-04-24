use lazy_static::lazy_static;
use reqwest::ClientBuilder;

lazy_static! {
    pub static ref REQWEST_CLIENT: reqwest::Client = ClientBuilder::new().build().unwrap();
}
