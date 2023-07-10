use diesel::{ExpressionMethods};
use crate::model::article::add_article::AddArticle;
use crate::model::article::add_article_content::AddArticleContent;
use crate::model::diesel::dolphin::custom_dolphin_models::{Article, ArticleContent};
use crate::{common::database::get_connection};
use crate::diesel::RunQueryDsl;

pub fn insert_article(input_article: &AddArticle, new_ontent: &AddArticleContent) -> Result<Article, diesel::result::Error>{
    use crate::model::diesel::dolphin::dolphin_schema::article::dsl::*;
    use crate::model::diesel::dolphin::dolphin_schema::article_content::dsl::*;

    let result = diesel::insert_into(article)
    .values(input_article)
    .on_conflict(title)
    .do_update()
    .set((
       title.eq("unknow"),
    ))
    .get_result::<Article>(&mut get_connection());

    let _result1 = diesel::insert_into(article_content)
    .values(new_ontent)
    .get_result::<ArticleContent>(&mut get_connection());

    return result;
}



