use diesel::Connection;
use feed_rs::{model::Feed, parser};
use log::error;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response,
};
use rss::Channel;
use futures::stream::StreamExt;

use crate::{
    common::database::get_connection,
    model::{
        article::{add_article::AddArticle, add_article_content::AddArticleContent},
        diesel::dolphin::custom_dolphin_models::{ArticleContent, RssSubSource},
    },
    service::article::{
        article_content_service::insert_article_content, article_service::insert_article,
    }, cache::redis_rss::{send_article_to_stream, async_send_article_to_stream},
};

pub async fn fetch_channel_article(source: RssSubSource) -> bool {
    let client = Client::new();
    let url: &str = &source.sub_url.clone();
    let response = client.get(url).headers(construct_headers()).send().await;
    match response {
        Ok(resp) => {
            return handle_channel_resp(resp, source).await;
        }
        Err(e) => {
            error!("pull channel facing error:{}", e);
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
                    return handle_rss_pull(body);
                }
                "ATOM" => {
                    let feed: Feed = parser::parse(body.as_bytes()).unwrap();
                    save_atom_channel_article(feed);
                    return false;
                }
                _ => {
                    error!("unknown rss type");
                    return false;
                }
            }
        }
        Err(err) => {
            error!("error,{}", err);
            return false;
        }
    }
}

fn handle_rss_pull(body: String) -> bool {
    let channel = Channel::read_from(body.as_bytes());
    match channel {
        Ok(channel_result) => {
            return save_rss_channel_article(channel_result);
        }
        Err(_) => {
            error!("error, pull rss channel error");
            return false;
        }
    }
}

// try to get the async result in future
async fn _async_save_rss_channel_article(channel: Channel) -> bool {
    if channel.items.is_empty() {
        return true;
    }
    let stream = futures::stream::iter(channel.items).map(|item| {
        let article: AddArticle = AddArticle::from_rss_entry(&item);
        let mut article_content = AddArticleContent::from_rss_entry(&item);
        
        async move {
            let result = save_article_impl(&article, &mut article_content);
            match result {
                Ok(_) => {
                    async_send_article_to_stream("pydolphin:stream:article").await;
                }
                Err(e) => {
                    error!("save article content error, {}", e);
                }
            }
        }
    });

    stream.for_each(|task| async {
        task.await;
    }).await;
    
    return true;
}

fn save_rss_channel_article(channel: Channel) -> bool {
    if channel.items.is_empty() {
        return true;
    }
    let mut success = true;
    channel.items.iter().for_each(|item| {
        let article: AddArticle = AddArticle::from_rss_entry(item);
        let mut article_content = AddArticleContent::from_rss_entry(item);
        let result = save_article_impl(&article, &mut article_content);
        match result {
            Ok(_) => {
                send_article_to_stream("pydolphin:stream:article");
            }
            Err(e) => {
                error!("save article content error,{}", e);
                success = false
            }
        }
    });
    return success;
}

fn save_atom_channel_article(feed: Feed) -> bool {
    if feed.entries.is_empty() {
        return true;
    }
    let mut success = true;
    feed.entries.iter().for_each(|item| {
        let _article: AddArticle = AddArticle::from_atom_entry(item);
        let mut article_content = AddArticleContent::from_atom_entry(item);
        let result = save_article_impl(&_article, &mut article_content);
        match result {
            Ok(_) => {}
            Err(e) => {
                error!("save single article content error,{}", e);
                success = false;
            }
        }
    });
    return success;
}

fn save_article_impl(
    add_article: &AddArticle,
    add_article_content: &mut AddArticleContent,
) -> Result<ArticleContent, diesel::result::Error> {
    let mut connection = get_connection();
    let result = connection.transaction(|_connection| {
        let add_result = insert_article(add_article);
        match add_result {
            Ok(inserted_article) => match inserted_article {
                Some(ia) => {
                    add_article_content.article_id = ia.id;
                    return insert_article_content(add_article_content);
                }
                None => todo!(),
            },
            Err(e) => {
                error!("insert article error,{}", e);
                diesel::result::QueryResult::Err(e)
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
