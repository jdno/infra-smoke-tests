use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    ClientBuilder,
};

pub struct TestClientBuilder;

impl TestClientBuilder {
    pub fn new() -> ClientBuilder {
        ClientBuilder::new().default_headers(HeaderMap::from_iter([(
            header::USER_AGENT,
            HeaderValue::from_static("infra-smoke-test"),
        )]))
    }
}
