use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::article::add_article::AddArticle;
use crate::model::diesel::dolphin::custom_dolphin_models::Article;
use diesel::ExpressionMethods;

pub fn insert_article(input_article: &AddArticle) -> Result<Article, diesel::result::Error> {
    use crate::model::diesel::dolphin::dolphin_schema::article::dsl::*;
    let result = diesel::insert_into(article)
        .values(input_article)
        .on_conflict(title)
        .do_update()
        .set((title.eq("unknow"),))
        .get_result::<Article>(&mut get_connection());
    return result;
}
