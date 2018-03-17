#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate diesel;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate chrono;

mod db;
mod schema;
mod static_files;
mod category;
mod post;

use rocket::Rocket;
use rocket_contrib::Template;
use rocket::response::{Redirect};
use chrono::prelude::*;
use category::{Category};
use post::{Post, DisplayPost};

#[get("/")]
fn index(conn: db::Conn) -> Template {
    #[derive(Debug, Serialize)]
    struct Context {
        categories: Vec<Category>,
        latest_posts: Vec<DisplayPost>
    }

    let categories = Category::all(&conn);
    let latest_posts = Post::latest(5, &conn);
    Template::render("index", &Context { categories, latest_posts })
}

#[get("/post/<slug>")]
fn post_view(slug: String, conn: db::Conn) -> Result<Template, Redirect> {

    #[derive(Debug, Serialize)]
    struct Context {
        categories: Vec<Category>,
        post: Post
    }

    let categories = Category::all(&conn);
    let result = Post::find_with_slug(slug.as_str(), &conn);

    if let Ok(post) = result {
        Ok(Template::render("post", &Context {categories,  post }))
    } else {
        Err(Redirect::to("/"))
    }
}

#[get("/category/<slug>")]
fn category_view(slug: String, conn: db::Conn) -> Template {
    #[derive(Debug, Serialize)]
    struct Context {
        categories: Vec<Category>,
        category: Category,
        posts: Vec<Post>
    };

    //todo remove unwrap and handle errors
    let category = Category::find_with_slug(slug.as_str(), &conn).unwrap();
    let posts = Post::find_with_category_id(category.id, &conn);
    let categories = Category::all(&conn);

    Template::render("category", &Context{ category, posts, categories })
}

#[get("/about-me")]
fn about() -> Template {
    #[derive(Debug, Serialize)]
    struct Context {};
    Template::render("about", Context{})
}

fn rocket() -> (Rocket) {
    let pool = db::init_pool();

    let rocket = rocket::ignite()
        .manage(pool)
        .mount("/", routes![index, post_view, about, category_view, static_files::all])
        .attach(Template::fairing());

    rocket
}

fn main() {
    rocket().launch();
}
