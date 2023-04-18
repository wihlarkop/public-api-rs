use reqwest::Client;
use serde::{Deserialize, Serialize};

//https://animechan.vercel.app/api

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeChanAPIResponse {
    pub anime: String,
    pub character: String,
    pub quote: String,
}


pub async fn _get_random_quote(client: &Client) -> Result<AnimeChanAPIResponse, reqwest::Error> {
    let url = format!("{}{}", &CONFIG.anime_chan, "/random");

    let resp = client.get(url)
        .send()
        .await?
        .json::<AnimeChanAPIResponse>()
        .await?;
    Ok(resp)
}

pub async fn _get_random_quote_by_anime_title(client: &Client, title: String) -> Result<AnimeChanAPIResponse, reqwest::Error> {
    let url: String = format!("{}{}", &CONFIG.anime_chan, "/random/anime");

    let resp = client.get(url)
        .query(&[("title", title)])
        .send()
        .await?
        .json::<AnimeChanAPIResponse>()
        .await?;

    Ok(resp)
}

pub async fn _get_random_quote_by_anime_character(client: &Client, name: String) -> Result<AnimeChanAPIResponse, reqwest::Error> {
    let url: String = format!("{}{}", &CONFIG.anime_chan, "/random/character");

    let resp = client.get(url)
        .query(&[("name", name)])
        .send()
        .await?
        .json::<AnimeChanAPIResponse>()
        .await?;

    Ok(resp)
}

pub async fn _get_ten_random_quotes(client: &Client) -> Result<Vec<AnimeChanAPIResponse>, reqwest::Error> {
    let url: String = format!("{}{}", &CONFIG.anime_chan, "/quotes");

    let resp = client.get(url)
        .send()
        .await?
        .json::<Vec<AnimeChanAPIResponse>>()
        .await?;

    Ok(resp)
}

pub async fn _get_ten_random_quotes_by_anime_title(client: &Client, title: String, page: Option<i32>) -> Result<Vec<AnimeChanAPIResponse>, reqwest::Error> {
    let url: String = format!("{}{}", &CONFIG.anime_chan, "/quotes/anime");

    let page = page.unwrap_or(1);

    let resp = client.get(url)
        .query(&[("title", title), ("page", page.to_string())])
        .send()
        .await?
        .json::<Vec<AnimeChanAPIResponse>>()
        .await?;

    Ok(resp)
}

pub async fn _get_ten_random_quotes_by_anime_character(client: &Client, name: String, page: Option<i32>) -> Result<Vec<AnimeChanAPIResponse>, reqwest::Error> {
    let url: String = format!("{}{}", &CONFIG.anime_chan, "/quotes/character");

    let page = page.unwrap_or(1);

    let resp = client.get(url)
        .query(&[("name", name), ("page", page.to_string())])
        .send()
        .await?
        .json::<Vec<AnimeChanAPIResponse>>()
        .await?;

    Ok(resp)
}

pub async fn _get_available_anime_names(client: &Client) -> Result<Vec<String>, reqwest::Error> {
    let url: String = format!("{}{}", &CONFIG.anime_chan, "/available/anime");

    let resp = client.get(url)
        .send()
        .await?
        .json::<Vec<String>>()
        .await?;

    Ok(resp)
}

pub async fn _get_available_anime_character(client: &Client) -> Result<Vec<String>, reqwest::Error> {
    let url: String = format!("{}{}", &CONFIG.anime_chan, "/available/character");

    let resp = client.get(url)
        .send()
        .await?
        .json::<Vec<String>>()
        .await?;

    Ok(resp)
}