table! {
    authors (id) {
        id -> Integer,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        avatar -> Nullable<Varchar>,
        alias -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Integer,
        name -> Varchar,
        description -> Text,
        url_key -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    category_post (category_id, post_id) {
        category_id -> Integer,
        post_id -> Integer,
    }
}

table! {
    posts (id) {
        id -> Integer,
        author_id -> Nullable<Integer>,
        title -> Varchar,
        body -> Text,
        url_key -> Varchar,
        published -> Bool,
        published_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(category_post -> categories (category_id));
joinable!(category_post -> posts (post_id));
joinable!(posts -> authors (author_id));

allow_tables_to_appear_in_same_query!(
    authors,
    categories,
    category_post,
    posts,
);
