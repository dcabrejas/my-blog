use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use chrono::NaiveDateTime;
use chrono::NaiveDate;

use schema::posts::dsl::{posts as all_posts};

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub url_key: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub published_at: NaiveDate
}

impl Post {
    pub fn find_with_id(id: i32, conn: &MysqlConnection) -> Result<Post, Error> {
        //all_posts.find(id).get_result::<Post>(conn)

        let post: Post = all_posts
            .find(id)
            .first(conn)
            .expect(&format!("Unable to find post {}", id));

        post
    }
}