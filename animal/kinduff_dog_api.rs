use reqwest::{Client, header, Url};
use reqwest::header::{HeaderMap, USER_AGENT};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct KinduffDogApiResponse {
    pub facts: Vec<String>,
    pub success: bool,
}

pub struct KinduffDogApi {
    _client: Client,
    _base_url: Url,
}

impl KinduffDogApi {
    pub fn new() -> KinduffDogApi {
        let base_url = Url::parse("https://dog-api.kinduff.com").unwrap();

        let user_agent = header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36");

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, user_agent);

        let client = Client::builder().default_headers(headers).build().unwrap();

        KinduffDogApi {
            _client: client,
            _base_url: base_url,
        }
    }

    pub async fn get_kinduff_dog(&self, number_of_fact: Option<i32>) -> Result<KinduffDogApiResponse, reqwest::Error> {
        let mut url = self._base_url.join("/api/facts").unwrap();

        let fact = number_of_fact.unwrap_or(1);

        url.query_pairs_mut().append_pair("number", &fact.to_string());

        let resp = self._client.get(url)
            .send()
            .await?
            .json::<KinduffDogApiResponse>()
            .await?;

        Ok(resp)
    }
}

