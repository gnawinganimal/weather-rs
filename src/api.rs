use reqwest::{Client, Error,};
use crate::weather::Current;
use dotenv_codegen::dotenv;

pub async fn get_current(location: String) -> Result<Current, Error> {
    let current: Current = Client::new()
        .get(format!(
            "http://api.weatherapi.com/v1/current.json?q={}&key={}",
            location,
            dotenv!("WEATHER_API_KEY")
        ))
        .send()
        .await?
        .json()
        .await?;

    Ok(current)
}
