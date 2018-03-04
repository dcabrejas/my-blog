#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate diesel;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod db;
mod static_files;
mod category;

use rocket::Rocket;
use rocket_contrib::Template;
use category::{Category};

#[get("/")]
fn index(conn: db::Conn) -> Template {
    #[derive(Debug, Serialize)]
    struct Context {
        categories: Vec<Category>
    }

    let categories = Category::all(&conn);
    Template::render("index", &Context { categories })
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name.as_str())
}

#[get("/category/<id>")]
fn category_view(id: u8) -> String {
    format!("Category id is : {}!", id)
}

#[get("/about")]
fn about() -> &'static str {
    "About me"
}

fn rocket() -> (Rocket) {
    let pool = db::init_pool();

    let rocket = rocket::ignite()
        .manage(pool)
        .mount("/", routes![index, hello, about, category_view, static_files::all])
        .attach(Template::fairing());

    rocket
}

fn main() {
    rocket().launch();
}
