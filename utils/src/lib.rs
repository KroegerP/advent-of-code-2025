use reqwest::blocking::Client;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};
use std::env;

pub fn get_input(year: u16, day: u8) -> String {
    dotenv::dotenv().expect("Failed to load .env file");

    let client = Client::new();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let mut headers: HeaderMap = HeaderMap::new();

    let session = format!(
        "session={}",
        env::var("SESSION").expect("Session token not set")
    );

    let cookie = HeaderValue::from_str(&session).unwrap();

    headers.insert(COOKIE, cookie);

    client
        .get(url)
        .headers(headers)
        .send()
        .expect("request failed")
        .text()
        .expect("body failed")
}
