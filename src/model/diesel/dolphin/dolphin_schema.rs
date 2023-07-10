table! {
    article (id) {
        id -> Int8,
        user_id -> Int8,
        title -> Varchar,
        author -> Varchar,
        guid -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        link -> Nullable<Varchar>,
        pub_time -> Nullable<Timestamptz>,
        sub_source_id -> Int8,
        cover_image -> Nullable<Varchar>,
        channel_reputation -> Int4,
        editor_pick -> Nullable<Int4>,
        permanent_store -> Int2,
    }
}

table! {
    article_content (id) {
        id -> Int8,
        article_id -> Int8,
        content -> Varchar,
    }
}

table! {
    rss_sub_source (id) {
        id -> Int8,
        sub_url -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        sub_status -> Int4,
        rss_type -> Varchar,
        standard_type -> Varchar,
        standard_version -> Varchar,
        cron -> Varchar,
        trigger_count -> Int4,
        next_trigger_time -> Nullable<Timestamp>,
        sub_name -> Varchar,
        last_trigger_time -> Nullable<Timestamptz>,
        source_url -> Nullable<Varchar>,
        sub_type -> Nullable<Varchar>,
        intro -> Nullable<Varchar>,
        remark -> Nullable<Varchar>,
        title_hash -> Nullable<Varchar>,
        failed_count -> Int4,
        lang -> Nullable<Varchar>,
        frequency_month -> Nullable<Int4>,
        reputation -> Nullable<Int8>,
        rep_latest_refresh_time -> Nullable<Int8>,
        scrapy_take_time -> Nullable<Int4>,
        follower -> Nullable<Int8>,
        censor_status -> Nullable<Int4>,
        etag -> Nullable<Varchar>,
        last_modified -> Nullable<Varchar>,
        editor_pick -> Nullable<Int4>,
        fav_icon_url -> Nullable<Varchar>,
        dynamic_interval -> Int4,
        local_icon_url -> Nullable<Varchar>,
        creator -> Int8,
        tags -> Jsonb,
        article_count -> Int8,
        article_count_latest_refresh_time -> Int8,
        comment_rss -> Int4,
        part_output -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    article,
    article_content,
    rss_sub_source,
);
