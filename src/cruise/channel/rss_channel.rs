use feed_rs::parser;
use log::error;
use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use rss::Channel;

pub async fn fetch_channel_article() {
    let client = Client::new();
    let url = "https://stackoverflow.com/feeds/user/2628868";
    let response = client.get(url).headers(construct_headers()).send().await.unwrap();
    let result = response.text().await;
    match result {
        Ok(body) => {
            let body_str = body.as_str();
            print!("{}",body_str);
            let channel1 = "<rss";
            if body_str.contains(channel1) {
                let channel = Channel::read_from(body.as_bytes());
                match channel {
                    Ok(c) => {
                        println!("Title: {}", c.title);
                        println!("Number of items: {}", c.items.len());
                    }
                    Err(e) => {
                        print!("error,{}", e)
                    }
                }
            } else if body_str.contains("<feed") {
                let feed = parser::parse(body.as_bytes()).unwrap();
                print!("atom");
            } else {
                error!("unknown sub format");
            }
        }
        Err(err) => {
            print!("error,{}", err)
        }
    }
}


fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36"));
    headers.insert("Accept", HeaderValue::from_static("text/html"));
    headers.insert("Referer", HeaderValue::from_static("https://www.google.com"));
    headers
}