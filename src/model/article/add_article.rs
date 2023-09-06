use crate::model::diesel::dolphin::custom_dolphin_models::RssSubSource;
use crate::model::diesel::dolphin::dolphin_schema::*;
use chrono::NaiveDateTime;
use feed_rs::model::Entry;
use log::error;
use rss::Item;
use rust_wheel::common::util::time_util::get_current_millisecond;
use serde::Deserialize;
use serde::Serialize;

use chrono::offset::Utc;
use chrono::DateTime;

#[derive(Insertable, Queryable, QueryableByName, Debug, Serialize, Deserialize, Default, Clone)]
#[diesel(table_name = article)]
pub struct AddArticle {
    pub user_id: i64,
    pub title: String,
    pub author: String,
    pub guid: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub link: Option<String>,
    pub pub_time: Option<DateTime<Utc>>,
    pub sub_source_id: i64,
    pub cover_image: Option<String>,
    pub channel_reputation: i32,
    pub editor_pick: Option<i32>,
    pub permanent_store: i16,
}

impl AddArticle {
    pub(crate) fn from_atom_entry(request: &Entry, rss_source: &RssSubSource) -> Self {
        let names: Vec<String> = request
            .authors
            .iter()
            .map(|person| person.name.clone())
            .collect();
        let names_concatenated = names.join(",");
        let article_pub_time = if request.published.is_some() {
            request.published
        } else {
            Some(Utc::now())
        };
        Self {
            user_id: 1,
            title: request.title.clone().unwrap().content,
            author: names_concatenated,
            guid: request.id.clone(),
            created_time: get_current_millisecond(),
            updated_time: get_current_millisecond(),
            link: request.links.first().map(|link| link.href.clone()),
            pub_time: article_pub_time,
            sub_source_id: rss_source.id,
            cover_image: Some("".to_owned()),
            channel_reputation: 0,
            editor_pick: rss_source.editor_pick,
            permanent_store: 0,
        }
    }

    pub(crate) fn from_rss_entry(request: &Item, rss_source: &RssSubSource) -> Self {
        let guid = request.guid.clone().unwrap_or_default();
        let mut article_pub_time = Some(Utc::now());
        if request.pub_date.is_some() {
            let one_of_time_str = &request.pub_date.clone().unwrap();
            let parsed_datetime =
                NaiveDateTime::parse_from_str(one_of_time_str, "%Y-%m-%d %H:%M:%S").or_else(|_| {
                    NaiveDateTime::parse_from_str(one_of_time_str, "%a, %d %b %Y %H:%M:%S %Z")
                }.or_else(|_| {
                    NaiveDateTime::parse_from_str(one_of_time_str, "%a, %d %b %Y %H:%M:%S %z")
                }.or_else(|_|{
                    NaiveDateTime::parse_from_str(one_of_time_str, "%Y-%m-%dT%H:%M:%S%z")
                }.or_else(|_|{
                    NaiveDateTime::parse_from_str(one_of_time_str, "%Y-%m-%d")
                }))));
            match parsed_datetime {
                Ok(parsed_pub_time) => {
                    let ndt = DateTime::<Utc>::from_utc(parsed_pub_time, Utc);
                    article_pub_time = Some(ndt);
                }
                Err(e) => {
                    let err_info: String = format!(
                        "Failed to parse rss datetime,time:{:?},error:{}",
                        request.pub_date.clone(),
                        e
                    );
                    error!("{}", err_info);
                }
            }
        } else {
            article_pub_time = Some(Utc::now());
        };
        Self {
            user_id: 1,
            title: request.title.clone().unwrap(),
            author: request.author.clone().unwrap_or_default(),
            guid: guid.value,
            created_time: get_current_millisecond(),
            updated_time: get_current_millisecond(),
            link: request.link.clone(),
            pub_time: article_pub_time,
            sub_source_id: rss_source.id,
            cover_image: Some("".to_owned()),
            channel_reputation: 0,
            editor_pick: rss_source.editor_pick,
            permanent_store: 0,
        }
    }
}
