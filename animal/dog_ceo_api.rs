use std::collections::HashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DogCeoAllBreedsAPIResponse {
    pub message: HashMap<String, Vec<String>>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DogCeoRandomImageAPIResponse {
    pub message: Vec<String>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DogCeoImageAPIResponse {
    pub message: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DogCeoSubBreedsAPIResponse {
    pub message: Vec<String>,
    pub status: String,
}

pub async fn _get_all_breeds(client: &Client) -> Result<DogCeoAllBreedsAPIResponse, reqwest::Error> {
    let resp = client.get("https://dog.ceo/api/breeds/list/all")
        .send()
        .await?
        .json::<DogCeoAllBreedsAPIResponse>()
        .await?;

    Ok(resp)
}

pub async fn _get_dog_random_image(client: &Client, dog_image_count: Option<i32>) -> Result<DogCeoImageAPIResponse, reqwest::Error> {
    let image_count = dog_image_count.unwrap_or(1);

    let url = format!("https://dog.ceo/api/breeds/image/random/{image_count}");

    let resp = client.get(url)
        .send()
        .await?
        .json::<DogCeoImageAPIResponse>()
        .await?;

    Ok(resp)
}

pub async fn _get_dog_random_image_by_breed(client: &Client, dog_breed: String, dog_image_count: Option<i32>) -> Result<DogCeoRandomImageAPIResponse, reqwest::Error> {
    let image_count = dog_image_count.unwrap_or(1);

    let dog_breed = dog_breed.to_lowercase();

    let url = format!("https://dog.ceo/api/breed/{dog_breed}/images/random/{image_count}");

    let resp = client.get(url)
        .send()
        .await?
        .json::<DogCeoRandomImageAPIResponse>()
        .await?;

    Ok(resp)
}

pub async fn _get_all_dog_random_image_by_breed(client: &Client, dog_breed: String) -> Result<DogCeoRandomImageAPIResponse, reqwest::Error> {
    let dog_breed = dog_breed.to_lowercase();

    let resp = client.get(format!("https://dog.ceo/api/breed/{dog_breed}/images"))
        .send()
        .await?
        .json::<DogCeoRandomImageAPIResponse>()
        .await?;

    Ok(resp)
}

pub async fn _get_dog_sub_breeds(client: &Client, dog_breed: String) -> Result<DogCeoRandomImageAPIResponse, reqwest::Error> {
    let dog_breed = dog_breed.to_lowercase();

    let resp = client.get(format!("https://dog.ceo/api/breed/{dog_breed}/list"))
        .send()
        .await?
        .json::<DogCeoRandomImageAPIResponse>()
        .await?;

    Ok(resp)
}
