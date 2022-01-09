#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod db;
mod schema;
mod utils;
mod path;

use rocket::{routes, Build, Rocket};
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use crate::db::user_table::{create_user_perm, DEFAULT_PATH, get_by_username};
use path::account::{home, users};
use crate::path::account::{register, register_post};
use crate::path::errors::not_found;

#[get("/")]
fn index() -> Template {
    Template::render(
        "home",
        context!(
            title: "Home",
            path: DEFAULT_PATH
        )
    )
}

#[launch]
fn rocket() -> Rocket<Build> {
    // Create a Admin account with perm if he doesn't exist
    if get_by_username("admin").is_none() {
        println!("Admin doesn't exist ! Creation of it ");
        if create_user_perm("admin", "password", true) == 0 {
            println!("Error at creation of the admin")
        };
    }
    rocket::build()
        .attach(Template::fairing())
        .register(
            "/",
            catchers![
                not_found,
            ],
        )
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/account/", routes![home, users, register, register_post])
}
