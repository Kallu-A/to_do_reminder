#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod db;
mod schema;

use rocket::{routes, Build, Rocket};

#[get("/")]
fn index() -> String {
    format!("Hello world !")
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        //.attach(Template::fairing())
        .mount("/", routes![index])
}
