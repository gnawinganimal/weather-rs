use crate::{weather::CurrentResponse, api::get_current};

pub mod weather;
pub mod api;

#[tokio::main]
async fn main() {
    let current: CurrentResponse = get_current(String::from("Paris")).await.unwrap();
    println!("{:?}", current);
}
