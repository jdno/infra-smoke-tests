use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    ClientBuilder,
};

/// Create a http client builder that with some default headers
pub fn test_http_client() -> ClientBuilder {
    reqwest::ClientBuilder::new().default_headers(HeaderMap::from_iter([(
        header::USER_AGENT,
        HeaderValue::from_static("infra-smoke-test"),
    )]))
}
