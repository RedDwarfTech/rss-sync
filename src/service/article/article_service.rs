use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::article::add_article::AddArticle;
use crate::model::diesel::dolphin::custom_dolphin_models::Article;
use crate::diesel::OptionalExtension;

pub fn insert_article(input_article: &AddArticle) -> Result<Option<Article>, diesel::result::Error> {
    use crate::model::diesel::dolphin::dolphin_schema::article::dsl::*;
    let result = diesel::insert_into(article)
        .values(input_article)
        .on_conflict_do_nothing()
        .get_result(&mut get_connection())
        .optional();
    return result;
}
