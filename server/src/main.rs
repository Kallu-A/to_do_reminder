#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate lettre;

use std::env;
use std::process::exit;

use rocket::fs::{relative, FileServer};
use rocket::http::{CookieJar, Status};
use rocket::request::FlashMessage;
use rocket::{routes, Build, Rocket};
use rocket_dyn_templates::Template;

use path::account::{home, users};
use path::todo::test;

use crate::db::user_table::{create_user_perm, get_by_username, DEFAULT_PATH};
use crate::path::account::{
    confirm_code, delete, edit, edit_post, form_password_change, home_logout, login, login_put,
    new_email, password_code, register, register_post, send_code, upload_picture,
};
use crate::path::errors::{
    expired_token, internal_error, method_not_allowed, not_found, not_login, token_match_none,
};
use crate::utils::cookie::handler_flash;
use crate::utils::email::verif_env;
use crate::utils::token::get_token;

use crate::utils::json::Data;
use dotenv::dotenv;

mod db;
mod path;
mod schema;
mod utils;

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

    let data = Data::get_json();
    let count_user = data.members;
    let count_todo = data.to_do;
    let average = if count_user == 0 {
        0f64
    } else {
        count_todo as f64 / count_user as f64
    };

    Template::render(
        "home",
        context!(
            title: "Home",
            path,
            color,
            message,
            count_user,
            count_todo,
            average
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
    dotenv().ok();
    if !verif_env() {
        println!("FATAL-ERROR: Your .env as incorrect SMTP value (HELP: maybe your relay doesn't allow the connection?)");
        exit(1);
    }

    // Create a Admin account with perm if he doesn't exist
    if get_by_username("admin").is_none() {
        println!("Admin doesn't exist ! Creation of it ");
        if create_user_perm(
            "admin",
            "password",
            env::var("ADRESS_SMTP")
                .expect("ADRESS_SMTP must be set")
                .as_str(),
            true,
        ) == 0
        {
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
                upload_picture,
                send_code,
                confirm_code,
                password_code,
                form_password_change,
                new_email
            ],
        )
        .mount("/to-do/", routes![test])
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;

    #[test]
    fn test_env() {
        dotenv().ok();
        assert!(env::var("DATABASE_URL").is_ok());
        assert!(env::var("SECRET_KEY").is_ok());
        assert!(env::var("TOKEN_KEY").is_ok());
        assert!(env::var("ADRESS_SMTP").is_ok());
        assert!(env::var("PASSWORD_SMTP").is_ok());
        assert!(env::var("RELAY_SMTP").is_ok());
        assert!(env::var("LAUNCH_MODE").is_ok());
    }
}
