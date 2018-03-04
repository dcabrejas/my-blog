use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

mod schema {
    table! {
        categories (id) {
            id -> Nullable<Integer>,
            name -> Text,
            description -> Text,
            url_key -> Text,
            created_at -> Nullable<Date>,
        }
    }
}

use self::schema::categories;
use self::schema::categories::dsl::{categories as all_categories};

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