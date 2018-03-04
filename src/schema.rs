table! {
    categories (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Text,
        url_key -> Text,
        created_at -> Nullable<Date>,
    }
}

table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
        published -> Bool,
        created_at -> Nullable<Date>,
        published_at -> Date,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    posts,
);
