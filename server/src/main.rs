#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, routes};


#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        //.attach(Template::fairing())
        .mount("/", routes![index])
}