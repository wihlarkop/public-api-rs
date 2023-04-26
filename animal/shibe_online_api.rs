use reqwest::{Client, header, Url};
use reqwest::header::{HeaderMap, USER_AGENT};
use serde::{Deserialize, Serialize};

pub struct ShibeOnlineApi {
    _client: Client,
    _base_url: Url,
}

impl ShibeOnlineApi {
    pub fn new() -> ShibeOnlineApi {
        let base_url = Url::parse("https://shibe.online").unwrap();

        let user_agent = header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36");

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, user_agent);

        let client = Client::builder().default_headers(headers).build().unwrap();

        ShibeOnlineApi {
            _client: client,
            _base_url: base_url,
        }
    }

    pub async fn get_shiba_images(&self, count: Option<i32>) -> Result<Vec<String>, reqwest::Error> {
        let mut url = self._base_url.join("/api/shibes").unwrap();

        let count = count.unwrap_or(1);

        url.query_pairs_mut().append_pair("count", &count.to_string());
        url.query_pairs_mut().append_pair("urls", &true.to_string());
        url.query_pairs_mut().append_pair("httpsUrls", &true.to_string());

        let resp = self._client.get(url)
            .send()
            .await?
            .json::<Vec<String>>()
            .await?;

        Ok(resp)
    }
}
