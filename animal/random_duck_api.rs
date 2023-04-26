use reqwest::{Client, header, Url};
use reqwest::header::{HeaderMap, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomDuckApiResponse {
    pub message: String,
    pub url: String,
}

pub struct RandomDuckApi {
    _client: Client,
    _base_url: Url,
}

impl RandomDuckApi {
    pub fn new() -> RandomDuckApi {
        let base_url = Url::parse("https://random-d.uk").unwrap();

        let user_agent = header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36");

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, user_agent);

        let client = Client::builder().default_headers(headers).build().unwrap();

        RandomDuckApi {
            _client: client,
            _base_url: base_url,
        }
    }

    pub async fn get_random_duck(&self) -> Result<RandomDuckApiResponse, reqwest::Error> {
        let mut url = self._base_url.join("/api/v2/random").unwrap();
        println!("{}", url);

        let resp = self._client.get(url)
            .send()
            .await?
            .json::<RandomDuckApiResponse>()
            .await?;

        Ok(resp)
    }
}