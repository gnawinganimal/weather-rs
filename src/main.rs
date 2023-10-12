use crate::{weather::Current, api::get_current};

pub mod weather;
pub mod api;

#[tokio::main]
async fn main() {
    let current: Current = get_current(String::from("Paris")).await.unwrap();
    println!("{:?}", current);
}
