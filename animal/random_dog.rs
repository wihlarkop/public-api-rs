use reqwest::{Client, header, Url};
use reqwest::header::{HeaderMap, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RandomDogApiResponse {
    #[serde(rename = "fileSizeBytes")]
    pub file_size_bytes: i64,
    pub url: String,
}

pub struct RandomDogApi {
    _client: Client,
    _base_url: Url,
}

impl RandomDogApi {
    pub fn new() -> RandomDogApi {
        let base_url = Url::parse("https://random.dog").unwrap();

        let user_agent = header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36");

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, user_agent);

        let client = Client::builder().default_headers(headers).build().unwrap();

        RandomDogApi {
            _client: client,
            _base_url: base_url,
        }
    }

    pub async fn get_random_dog(&self) -> Result<RandomDogApiResponse, reqwest::Error> {
        let mut url = self._base_url.join("/woof.json").unwrap();

        let resp = self._client.get(url)
            .send()
            .await?
            .json::<RandomDogApiResponse>()
            .await?;

        Ok(resp)
    }
}
