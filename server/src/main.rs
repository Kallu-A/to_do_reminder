#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod db;

use rocket::{Build, Rocket, routes};

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



