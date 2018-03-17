use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use chrono::NaiveDateTime;

//use post::Post;

use schema::categories;
use schema::categories::dsl::{categories as all_categories, url_key};
//use schema::posts::dsl::{posts as all_posts};
//use schema::category_post::dsl::{category_post as category_post_map};

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub url_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct CategoryPostMap {
    pub category_id: i32,
    pub post_id: i32
}

impl Category {
    pub fn all(conn: &MysqlConnection) -> Vec<Category> {
        all_categories.order(categories::id.desc()).load::<Category>(conn).unwrap()
    }

    pub fn find_with_id(id: i32, conn: &MysqlConnection) -> Result<Category, Error> {
        all_categories.find(id).get_result::<Category>(conn)
    }

    pub fn find_with_slug(slug: &str, conn: &MysqlConnection) -> Result<Category, Error> {
        all_categories
            .filter(url_key.eq(slug))
            .first::<(Category)>(conn)
    }

//    pub fn find_with_id(id: i32, conn: &MysqlConnection) {
//        let data = all_categories
//            .find(id)
//            .left_join(category_post_map.left_join(all_posts))
//            .load::<(Category, (Option<CategoryPostMap>, Option<Post>))>(conn);
//        panic!("{:?}", data);
//    }
}