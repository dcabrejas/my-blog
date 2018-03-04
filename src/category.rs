use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use schema::categories;
use schema::categories::dsl::{categories as all_categories};

#[table_name="categories"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Category {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub url_key: String,
    pub created_at: Option<String>
}

impl Category {
    pub fn all(conn: &SqliteConnection) -> Vec<Category> {
        all_categories.order(categories::id.desc()).load::<Category>(conn).unwrap()
    }
}