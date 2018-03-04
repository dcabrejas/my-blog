use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::result::Error;

use schema::posts;
use schema::posts::dsl::{posts as all_posts};

#[table_name="posts"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: Option<String>,
    pub published_at: String
}

impl Post {
    pub fn find_with_id(id: i32, conn: &SqliteConnection) -> Result<Post, Error> {
        all_posts.find(id).get_result::<Post>(conn)
    }
}