table! {
    categories (id) {
        id -> Integer,
        name -> Varchar,
        description -> Varchar,
        url_key -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        url_key -> Varchar,
        published -> Bool,
        published_at -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    posts,
);
