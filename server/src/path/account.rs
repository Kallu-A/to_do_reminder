use regex::Regex;
use rocket::form::Form;
use rocket::http::{CookieJar, Status};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::Template;
use crate::{context, get_by_username};
use crate::db::user_table::{create_user, DEFAULT_PATH, get_all, UsersForm, UsersLogin};
use crate::utils::cookie::{cookie_handler, handler_flash};
use crate::utils::token::get_token;

#[get("/")]
pub fn home() -> Redirect {
    Redirect::to("login")
}

#[get("/register")]
pub fn register(jar: &CookieJar<'_>, flash: Option<FlashMessage>) -> Result<Template, Status> {
    let (color, message) = handler_flash(flash);
    match get_token(jar) {
        Ok(_) => Err(Status::MethodNotAllowed),
        Err(_) => {
            let username_x = cookie_handler(jar, "username_x".to_string());
            let password_first = cookie_handler(jar, "password_x.first".to_string());
            let password_second = cookie_handler(jar, "password_x.second".to_string());
            Result::Ok(Template::render(
                "account/register",
                context!(
                    title: "Register",
                ),
            ))
        }
    }
}

#[post("/register", data = "<form>")]
pub fn register_post(
    jar: &CookieJar<'_>,
    form: Form<UsersForm>,
) -> Result<Redirect, Flash<Redirect>> {

    if form.username_x.is_empty() {
        return Result::Err(Flash::error(
            Redirect::to("register"),
            "rYou need a Username",
        ));
    }
    let regex = Regex::new("^test*").unwrap();
    let regex2 = Regex::new("#-#").unwrap();
    if form.username_x == "default.png"
        || form.username_x == "admin"
        || regex.is_match(form.username_x)
        || regex2.is_match(form.username_x)
    {
        return Result::Err(Flash::error(
            Redirect::to("register"),
            "rReserved username sorry",
        ));
    }

    if form.password_x.first.is_empty() {
        return Result::Err(Flash::error(
            Redirect::to("register"),
            "rYou must choose a password",
        ));
    }

    if form.password_x.second.is_empty() {
        return Result::Err(Flash::error(
            Redirect::to("register"),
            "rYou have to fill the second password",
        ));
    }

    if form.password_x.first != form.password_x.second {
        return Result::Err(Flash::error(
            Redirect::to("register"),
            "rYou're password doesn't match",
        ));
    }

    let user_exist = get_by_username(form.username_x);

    match user_exist {
        Some(_) => {
            Result::Err(Flash::error(
                Redirect::to("register"),
                "rUser already exists",
            ))
        }
        None => {
            create_user(form.username_x, form.password_x.first);
            Result::Ok(Redirect::to("register"))
        }
    }
}

#[get("/login")]
pub fn login() -> Template {
    Template::render("account/home", context!(
        title: "Home",
        path: DEFAULT_PATH
    ))
}


#[put("/login", data = "<form>")]
pub fn login_put(form: Form<UsersLogin<'_>>) -> Flash<Redirect> {
    if let Some(s) = get_by_username(form.username_x) {
        return if s.password == form.password_x {
            Flash::success(Redirect::to("/account"), "")
        } else {
            Flash::error(Redirect::to("/account"), "rWrong password")
        };
    }
    Flash::error(Redirect::to("/account"), "rUsername doesn't exist ")
}

#[get("/users")]
pub fn users() -> Template {
    let users = get_all();
    Template::render(
        "account/users",
        context!(
            title: "List of users",
            path: DEFAULT_PATH,
            users,
        )
    )
}