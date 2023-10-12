use reqwest::{Client, Error,};
use crate::weather::{GetCurrent, GetForecast};
use dotenv_codegen::dotenv;

pub async fn get_current(location: String) -> Result<GetCurrent, Error> {
    let current: GetCurrent = Client::new()
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

pub async fn get_forecast(location: String, days: u32) -> Result<GetForecast, Error> {
    let forecast: GetForecast = Client::new()
        .get(format!(
            "http://api.weatherapi.com/v1/forecast.json?q={}&days={}&key={}",
            location,
            days,
            dotenv!("WEATHER_API_KEY")
        ))
        .send()
        .await?
        .json()
        .await?;

    Ok(forecast)
}
