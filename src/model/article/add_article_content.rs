use feed_rs::model::Entry;
use rss::Item;
use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = article_content)]
pub struct AddArticleContent {
    pub article_id: i64,
    pub content: String,
}

impl AddArticleContent {
    pub(crate) fn from_atom_entry(request: &Entry) ->Self {
        Self {
            article_id: 1,
            content: request.content.clone().unwrap_or_default().body.unwrap(),
        }
    }

    pub(crate) fn from_rss_entry(request: &Item) ->Self {
        Self {
            article_id: 1,
            content: request.content.clone().unwrap(),
        }
    }
}