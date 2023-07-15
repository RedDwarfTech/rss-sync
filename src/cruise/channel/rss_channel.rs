use crate::{
    common::database::get_connection,
    model::{
        article::{add_article::AddArticle, add_article_content::AddArticleContent},
        diesel::dolphin::custom_dolphin_models::{ArticleContent, RssSubSource},
    },
    service::{
        article::{
            article_content_service::insert_article_content, article_service::insert_article,
        },
        channel::channel_service::update_substatus,
    },
};
use diesel::Connection;
use feed_rs::{model::Feed, parser};
use log::{error, info, warn};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response,
};
use rss::Channel;
use rust_wheel::config::cache::redis_util::{set_value, sync_get_str};

pub async fn fetch_channel_article(source: RssSubSource) -> bool {
    // https://stackoverflow.com/questions/65977261/how-can-i-accept-invalid-or-self-signed-ssl-certificates-in-rust-futures-reqwest
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap_or_default();
    let url: &str = &source.sub_url.clone();
    let response = client.get(url).headers(construct_headers()).send().await;
    match response {
        Ok(resp) => {
            return handle_channel_resp(resp, source).await;
        }
        Err(e) => {
            error!("pull channel facing error:{}", e);
            if e.to_string().contains("dns error") {
                warn!("handle dns issue,{}", e.status().unwrap_or_default());
                let _result = update_substatus(source, -1);
                return true;
            }
            if e.to_string()
                .contains("connection closed before message completed")
            {
                warn!(
                    "handle could not connect issue,code:{:?},error:{}",
                    e.status(),
                    e
                );
                let _result = update_substatus(source, -1);
                return true;
            }
            error!("http get channel info facing error,{}", e);
            return false;
        }
    }
}

async fn handle_channel_resp(response: Response, source: RssSubSource) -> bool {
    let result = response.text().await;
    match result {
        Ok(body) => {
            let rss_type_str = &source.rss_type;
            match rss_type_str.as_str() {
                "RSS" => {
                    return handle_rss_pull(body, source);
                }
                "ATOM" => {
                    return handle_atom_pull(body, source);
                }
                _ => {
                    let channel_json = serde_json::to_string(&source);
                    error!(
                        "unknown rss type, channel: {}",
                        channel_json.unwrap_or_default()
                    );
                    let _result = update_substatus(source, -6);
                    return true;
                }
            }
        }
        Err(err) => {
            error!("get http response error,{}", err);
            return false;
        }
    }
}

fn handle_atom_pull(body: String, pull_channel: RssSubSource) -> bool {
    let feed: Feed = parser::parse(body.as_bytes()).unwrap();
    return save_atom_channel_article(feed, &pull_channel);
}

fn handle_rss_pull(body: String, pull_channel: RssSubSource) -> bool {
    let channel = Channel::read_from(body.as_bytes());
    match channel {
        std::result::Result::Ok(channel_result) => {
            return save_rss_channel_article(channel_result, &pull_channel);
        }
        Err(e) => {
            let channel_json = serde_json::to_string(&pull_channel);
            error!(
                "error, parse rss channel{} error {},the body is: {}",
                channel_json.unwrap_or_default(),
                e,
                body
            );
            let _result = update_substatus(pull_channel, -5);
            return true;
        }
    }
}

fn save_rss_channel_article(channel: Channel, rss_source: &RssSubSource) -> bool {
    if channel.items.is_empty() {
        return true;
    }
    let mut success = true;
    channel.items.iter().for_each(|item| {
        let article: AddArticle = AddArticle::from_rss_entry(item, &rss_source);
        let article_content = AddArticleContent::from_rss_entry(item);
        success = pre_check(&article, rss_source, article_content);
    });
    return success;
}

fn save_atom_channel_article(feed: Feed, rss_source: &RssSubSource) -> bool {
    if feed.entries.is_empty() {
        return true;
    }
    let mut success = true;
    feed.entries.iter().for_each(|item| {
        let _article: AddArticle = AddArticle::from_atom_entry(item, rss_source);
        let article_content = AddArticleContent::from_atom_entry(item);
        success = pre_check(&_article, rss_source, article_content);
    });
    return success;
}

fn pre_check(
    _article: &AddArticle,
    rss_source: &RssSubSource,
    mut article_content: AddArticleContent,
) -> bool {
    let article_cached_key = format!(
        "{}{}{}",
        "pydolphin:article:pull:cache:", rss_source.id, _article.title
    );
    let mut success = true;
    let cached_article = sync_get_str(&article_cached_key).unwrap();
    match cached_article {
        Some(_) => {}
        None => {
            let result = save_article_impl(&_article, &mut article_content);
            match result {
                std::result::Result::Ok(_ac) => {
                    let content = serde_json::to_string(&article_content).unwrap();
                    let err_info = format!("save article failed.{}", content);
                    set_value(&article_cached_key, &content, 259200).expect(&err_info);
                    info!("save article success")
                }
                Err(e) => {
                    error!("save article failed,{}", e);
                    success = false;
                }
            }
        }
    }
    return success;
}

fn save_article_impl(
    add_article: &AddArticle,
    add_article_content: &mut AddArticleContent,
) -> Result<ArticleContent, diesel::result::Error> {
    let mut connection = get_connection();
    let result: Result<ArticleContent, diesel::result::Error> =
        connection.transaction(|_connection| {
            let add_result = insert_article(add_article)?;
            match add_result {
                Some(ia) => {
                    add_article_content.article_id = ia.id;
                    return insert_article_content(add_article_content);
                }
                None => {
                    return Ok(ArticleContent::default());
                }
            }
        });
    return result;
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
