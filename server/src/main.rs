#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod db;
mod path;
mod schema;
mod utils;

use crate::db::user_table::{create_user_perm, get_by_username, DEFAULT_PATH};
use crate::path::account::{
    delete, edit, edit_post, home_logout, login, login_put, register, register_post, upload_picture,
};
use crate::path::errors::{
    expired_token, internal_error, method_not_allowed, not_found, not_login, token_match_none,
};
use crate::utils::cookie::handler_flash;
use crate::utils::token::get_token;
use path::account::{home, users};
use rocket::fs::{relative, FileServer};
use rocket::http::{CookieJar, Status};
use rocket::request::FlashMessage;
use rocket::{routes, Build, Rocket};
use rocket_dyn_templates::Template;
use std::env;

/// Home of the website
/// handle flash message
#[get("/")]
fn index(jar: &CookieJar<'_>, flash: Option<FlashMessage>) -> Template {
    let (color, message) = handler_flash(flash);

    let path = if let Ok(user) = get_token(jar) {
        user.get_path()
    } else {
        DEFAULT_PATH.to_string()
    };

    Template::render(
        "home",
        context!(
            title: "Home",
            path,
            color,
            message
        ),
    )
}

/// Path to try a status code put in dynamic arg to see how is looking if his work
#[get("/status/<code>")]
fn status(jar: &CookieJar<'_>, code: u16) -> Result<Status, Template> {
    let path = if let Ok(user) = get_token(jar) {
        user.get_path()
    } else {
        DEFAULT_PATH.to_string()
    };

    if let Some(status) = Status::from_code(code) {
        Result::Ok(status)
    } else {
        Result::Err(Template::render(
            "error/wrong_status",
            context!(
                title: "Wrong Status",
                path,
                code,
            ),
        ))
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    // Create a Admin account with perm if he doesn't exist
    if get_by_username("admin").is_none() {
        println!("Admin doesn't exist ! Creation of it ");
        if create_user_perm("admin", "password", true) == 0 {
            println!("Error at creation of the admin");
        };
    }
    rocket::build()
        .attach(Template::fairing())
        .register(
            "/",
            catchers![
                not_found,
                method_not_allowed,
                not_login,
                expired_token,
                token_match_none,
                internal_error
            ],
        )
        .mount("/", routes![index, status])
        .mount("/static", FileServer::from(relative!("static")))
        .mount(
            "/account/",
            routes![
                home,
                users,
                register,
                register_post,
                login,
                login_put,
                home_logout,
                delete,
                edit,
                edit_post,
                upload_picture
            ],
        )
}
