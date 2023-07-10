
use diesel::Connection;
use feed_rs::{parser, model::Feed};
use log::{error};
use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use rss::Channel;

use crate::{model::{ article::{add_article::AddArticle, add_article_content::AddArticleContent}}, common::database::get_connection, service::article::article_service::insert_article};

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
                let channel = Channel::read_from(body.as_bytes()).unwrap();
                _save_rss_channel_article(channel);
            } else if body_str.contains("<feed") {
                let _feed:Feed = parser::parse(body.as_bytes()).unwrap();
                _save_atom_channel_article(_feed);
            } else {
                error!("unknown sub format");
            }
        }
        Err(err) => {
            print!("error,{}", err)
        }
    }
}

fn _save_rss_channel_article(_channel: Channel) {
    if _channel.items.is_empty() {
        return;
    }
    _channel.items.iter().for_each(|item| {
        println!("{}", "item.title");
        let _article: AddArticle = AddArticle::_from_rss_entry(item);
        let article_content = AddArticleContent::_from_rss_entry(item);
        save_article_impl(&_article, &article_content);
    });
}

fn _save_atom_channel_article(feed: Feed) {
    if feed.entries.is_empty() {
        return;
    }
    feed.entries.iter().for_each(|item| {
        println!("{}", "item.title");
        let _article: AddArticle = AddArticle::_from_atom_entry(item);
        let article_content = AddArticleContent::_from_atom_entry(item);
        save_article_impl(&_article, &article_content);
    });
}

fn save_article_impl(add_article: &AddArticle, add_article_content: &AddArticleContent) {
    let mut connection = get_connection();
    let _result = connection.transaction(|_connection| {
        let add_result = insert_article(add_article,add_article_content);
        return add_result;
    });
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36"));
    headers.insert("Accept", HeaderValue::from_static("text/html"));
    headers.insert("Referer", HeaderValue::from_static("https://www.google.com"));
    headers
}