use reqwest::{Client, header};
use reqwest::header::{HeaderMap, USER_AGENT};

pub async fn init_client() -> Client {
    let user_agent = header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36");

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, user_agent);

    Client::builder().default_headers(headers).build().unwrap()
}