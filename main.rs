use crate::animal::kinduff_dog_api::KinduffDogApi;
use crate::animal::meow_fact_api::MeowFactApi;

mod animal;
mod client;
mod anime;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MeowFactApi::new();
    let resp = client.get_meow_fact(None, Some(3)).await.unwrap();
    println!("{:#?}", resp);
    Ok(())
}

