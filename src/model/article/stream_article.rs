
use feed_rs::model::Entry;
use rss::Item;
use rust_wheel::common::util::time_util::get_current_millisecond;
use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;

use chrono::DateTime;
use chrono::offset::Utc;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = article)]
pub struct AddArticle {
    pub id: i64,
    pub sub_source_id: i64
}

impl AddArticle {
    pub(crate) fn from(id: i64, sub_source_id: i64) -> Self {
        Self {
            id: id,
            sub_source_id: sub_source_id,
        }
    }

}