use crate::{
    api::{get_current, get_forecast},
    weather::{GetCurrent, GetForecast},
};

pub mod api;
pub mod weather;

#[tokio::main]
async fn main() {
    let current: GetCurrent = get_current("Corvallis".to_string()).await.unwrap();
    println!("{:?}", current);

    let forecast: GetForecast = get_forecast("Corvallis".to_string(), 1).await.unwrap();
    println!("{:?}", forecast);
}
