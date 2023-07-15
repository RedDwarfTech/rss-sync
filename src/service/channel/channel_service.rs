use std::str::FromStr;
use std::time::SystemTime;

use crate::diesel::RunQueryDsl;
use crate::{
    common::database::get_connection, model::diesel::dolphin::custom_dolphin_models::RssSubSource,
};
use chrono::{NaiveDateTime, Utc};
use cron::Schedule;
use diesel::{ExpressionMethods, QueryDsl};
use rust_wheel::common::util::time_util::get_current_millisecond;

pub fn get_channel_by_id(channel_id: i64) -> Vec<RssSubSource> {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source as channel_table;
    let mut query = channel_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(channel_table::id.eq(channel_id));
    let cvs = query
        .load::<RssSubSource>(&mut get_connection())
        .expect("error get channel list");
    return cvs;
}

pub fn get_fresh_channel() -> Vec<RssSubSource> {
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

pub fn update_substatus(
    channel: RssSubSource,
    new_sub_status: i32,
) -> Result<RssSubSource, diesel::result::Error> {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let predicate =
        crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::id.eq(channel.id);
    let update_result = diesel::update(rss_sub_source.filter(predicate))
        .set(sub_status.eq(new_sub_status))
        .get_result::<RssSubSource>(&mut get_connection());
    return update_result;
}

pub fn update_pulled_channel(channel: RssSubSource) -> Result<RssSubSource, diesel::result::Error> {
    use crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::dsl::*;
    let predicate =
        crate::model::diesel::dolphin::dolphin_schema::rss_sub_source::id.eq(channel.id);
    let next_time = next_execution_time(&channel.cron);
    let update_result = diesel::update(rss_sub_source.filter(predicate))
        .set((
            next_trigger_time.eq(next_time),
            updated_time.eq(get_current_millisecond()),
        ))
        .get_result::<RssSubSource>(&mut get_connection());
    return update_result;
}

fn next_execution_time(cron_expression: &str) -> NaiveDateTime {
    let schedule = Schedule::from_str(cron_expression).unwrap();
    let mut next_trigger = schedule.upcoming(Utc).take(1);
    let next_trigger_time = next_trigger.next();
    let naive_datetime: NaiveDateTime = next_trigger_time.unwrap().naive_utc();
    return naive_datetime;
}
