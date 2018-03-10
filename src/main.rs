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
use category::{Category};
use post::{Post};

#[get("/")]
fn index(conn: db::Conn) -> Template {
    #[derive(Debug, Serialize)]
    struct Context {
        categories: Vec<Category>
    }

    let categories = Category::all(&conn);
    Template::render("index", &Context { categories })
}

#[get("/post/<id>")]
fn post_view(id: i32, conn: db::Conn) -> Result<Template, Redirect> {

    #[derive(Debug, Serialize)]
    struct Context {
        post: Post
    }

    let result = Post::find_with_id(id, &conn);

    if let Ok(post) = result {
        Ok(Template::render("post", &Context { post }))
    } else {
        Err(Redirect::to("/"))
    }
}

#[get("/category/<id>")]
fn category_view(id: u8) -> String {
    format!("Category id is : {}!", id)
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
        //.mount("/", routes![index, post_view, about, category_view, static_files::all])
        .mount("/", routes![index, about, category_view, static_files::all])
        .attach(Template::fairing());

    rocket
}

fn main() {
    rocket().launch();
}
