use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;
use chrono::NaiveDateTime;

use category::CategoryPostMap;

use schema::posts::dsl::{posts as all_posts, published, published_at, url_key};
use schema::category_post::dsl::{category_post as category_post_map, category_id as map_cat_id};

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Post {
    pub id: i32,
    pub author_id: Option<i32>,
    pub title: String,
    pub body: String,
    pub url_key: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub published_at: NaiveDateTime
}

#[derive(Serialize, Debug)]
pub struct DisplayPost {
    pub title: String,
    pub body: String,
    pub url_key: String,
    pub published_at: String
}

impl Post {
    pub fn find_with_id(id: i32, conn: &MysqlConnection) -> Result<Post, Error> {
        all_posts.find(id).get_result::<Post>(conn)
    }

    pub fn find_with_slug(slug: &str, conn: &MysqlConnection) -> Result<Post, Error> {
        all_posts
            .filter(url_key.eq(slug))
            .first::<(Post)>(conn)
    }

    pub fn latest(limit: i64, conn: &MysqlConnection) -> Vec<DisplayPost> {
        let latest_posts = all_posts
            .filter(published.eq(true))
            .order(published_at.asc())
            .limit(limit)
            .load::<Post>(conn)
            .expect("Error loading latest posts");

        latest_posts.into_iter().map(|post: (Post) | DisplayPost {
            title: post.title,
            body: post.body,
            url_key: post.url_key,
            //Sunday, August 31, 2014 at 6:00 p.m
            published_at: post.published_at.format("%A, %B %e, %Y at %H:%M %P").to_string()
        }).collect()
    }

    pub fn find_with_category_id(id: i32, conn: &MysqlConnection) -> Vec<Post>{
        let result = all_posts
            .inner_join(category_post_map)
            .filter(map_cat_id.eq(id))
            .filter(published.eq(true))
            .order(published_at.asc())
            .load::<(Post, CategoryPostMap)>(conn)
            .expect("Error loading posts by category");

        let posts = result.into_iter().map(|x: (Post, CategoryPostMap) | x.0 ).collect();

        posts
    }
}