use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use chrono::NaiveDateTime;

use schema::categories;
use schema::categories::dsl::{categories as all_categories};

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub url_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl Category {
    pub fn all(conn: &MysqlConnection) -> Vec<Category> {
        all_categories.order(categories::id.desc()).load::<Category>(conn).unwrap()
    }
}