use reqwest::{Client, header, Url};
use reqwest::header::{HeaderMap, USER_AGENT};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct MeowFactAPIResponse {
    pub data: Vec<String>,
}

pub struct MeowFactApi {
    _client: Client,
    _base_url: Url,
}

// TODO
// 1. Implement filter with language support

impl MeowFactApi {
    pub fn new() -> MeowFactApi {
        let base_url = Url::parse("https://meowfacts.herokuapp.com").unwrap();

        let user_agent = header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36");

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, user_agent);

        let client = Client::builder().default_headers(headers).build().unwrap();

        MeowFactApi {
            _client: client,
            _base_url: base_url,
        }
    }

    pub async fn get_meow_fact(&self, fact_count: Option<i32>, id: Option<i32>) -> Result<MeowFactAPIResponse, reqwest::Error> {
        let mut url = self._base_url.clone();

        if let Some(count) = fact_count {
            url.query_pairs_mut().append_pair("count", &count.to_string());
        }

        if let Some(id) = id {
            url.query_pairs_mut().append_pair("id", &id.to_string());
        }

        let resp = self._client.get(url)
            .send()
            .await?
            .json::<MeowFactAPIResponse>()
            .await?;

        Ok(resp)
    }
}