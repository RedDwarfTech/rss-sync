use diesel::PgConnection;
use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::model::article::add_article_content::AddArticleContent;
use crate::model::diesel::dolphin::custom_dolphin_models::ArticleContent;

pub fn insert_article_content(
    new_ontent: &AddArticleContent,
) -> Result<ArticleContent, diesel::result::Error> {
    use crate::model::diesel::dolphin::dolphin_schema::article_content::dsl::*;
    let result = diesel::insert_into(article_content)
        .values(new_ontent)
        .get_result::<ArticleContent>(&mut get_connection());
    return result;
}

pub fn trans_insert_article_content(
    new_ontent: &AddArticleContent,
    conn: &mut PgConnection
) -> Result<ArticleContent, diesel::result::Error> {
    use crate::model::diesel::dolphin::dolphin_schema::article_content::dsl::*;
    let result = diesel::insert_into(article_content)
        .values(new_ontent)
        .get_result::<ArticleContent>(conn);
    return result;
}