use std::time::SystemTime;

use diesel::{ExpressionMethods, QueryDsl};
use crate::{common::database::get_connection, model::diesel::dolphin::custom_dolphin_models::RssSubSource};
use crate::diesel::RunQueryDsl;

pub fn get_channel_by_id(channel_id: i64) -> Vec<RssSubSource>{
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source as channel_table;
    let mut query = channel_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(channel_table::id.eq(channel_id));
    let cvs = query
        .load::<RssSubSource>(&mut get_connection())
        .expect("error get channel list");
    return cvs;
}

pub fn get_fresh_channel() -> Vec<RssSubSource>{
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source as channel_table;
    let mut query = channel_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(channel_table::sub_status.eq(1));
    query = query.filter(channel_table::censor_status.eq(1));
    query = query.filter(channel_table::next_trigger_time.lt(SystemTime::now()));
    let cvs = query
        .limit(1)
        .load::<RssSubSource>(&mut get_connection())
        .expect("error get ready sub channel list");
    return cvs;
}