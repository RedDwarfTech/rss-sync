use diesel::Connection;
use feed_rs::{model::Feed, parser};
use log::error;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use rss::Channel;

use crate::{
    common::database::get_connection,
    model::article::{add_article::AddArticle, add_article_content::AddArticleContent},
    service::article::{article_service::insert_article, article_content_service::insert_article_content},
};

pub async fn fetch_channel_article() {
    let client = Client::new();
    let url = "https://stackoverflow.com/feeds/user/2628868";
    let response = client
        .get(url)
        .headers(construct_headers())
        .send()
        .await
        .unwrap();
    let result = response.text().await;
    match result {
        Ok(body) => {
            let body_str = body.as_str();
            let channel1 = "<rss";
            if body_str.contains(channel1) {
                let channel = Channel::read_from(body.as_bytes()).unwrap();
                save_rss_channel_article(channel);
            } else if body_str.contains("<feed") {
                let feed: Feed = parser::parse(body.as_bytes()).unwrap();
                save_atom_channel_article(feed);
            } else {
                error!("unknown sub format");
            }
        }
        Err(err) => {
            print!("error,{}", err)
        }
    }
}

fn save_rss_channel_article(channel: Channel) {
    if channel.items.is_empty() {
        return;
    }
    channel.items.iter().for_each(|item| {
        let article: AddArticle = AddArticle::from_rss_entry(item);
        let mut article_content = AddArticleContent::from_rss_entry(item);
        save_article_impl(&article, &mut article_content);
    });
}

fn save_atom_channel_article(feed: Feed) {
    if feed.entries.is_empty() {
        return;
    }
    feed.entries.iter().for_each(|item| {
        let _article: AddArticle = AddArticle::from_atom_entry(item);
        let mut article_content = AddArticleContent::from_atom_entry(item);
        save_article_impl(&_article, &mut article_content);

    });
}

fn save_article_impl(add_article: &AddArticle, add_article_content: &mut AddArticleContent) {
    let mut connection = get_connection();
    let _result = connection.transaction(|_connection| {
        let add_result = insert_article(add_article);
        match add_result {
            Ok(inserted_article) => {
                add_article_content.article_id = inserted_article.id;
                return insert_article_content(add_article_content);
            },
            Err(e) => {
                diesel::result::QueryResult::Err(e)
            },
        }
    });
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36"));
    headers.insert("Accept", HeaderValue::from_static("text/html"));
    headers.insert(
        "Referer",
        HeaderValue::from_static("https://www.google.com"),
    );
    headers
}
