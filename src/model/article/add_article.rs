
use feed_rs::model::Entry;
use rss::Item;
use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;

use chrono::DateTime;
use chrono::offset::Utc;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
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
    pub(crate) fn _from_atom_entry(request: &Entry) ->Self {
        Self {
            user_id: 1,
            title: request.title.clone().unwrap().content,
            author: "".to_owned(),
            guid:  "".to_owned(),
            created_time: 1,
            updated_time:1,
            link: Some("".to_owned()),
            pub_time: Some(Utc::now()),
            sub_source_id: 1,
            cover_image: Some("".to_owned()),
            channel_reputation: 0,
            editor_pick: Some(0),
            permanent_store: 0,
        }
    }

    pub(crate) fn _from_rss_entry(request: &Item) ->Self {
        Self {
            user_id: 1,
            title: request.title.clone().unwrap(),
            author: "".to_owned(),
            guid:  "".to_owned(),
            created_time: 1,
            updated_time:1,
            link: Some("".to_owned()),
            pub_time: Some(Utc::now()),
            sub_source_id: 1,
            cover_image: Some("".to_owned()),
            channel_reputation: 0,
            editor_pick: Some(0),
            permanent_store: 0,
        }
    }
}