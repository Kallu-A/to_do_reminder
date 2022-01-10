use crate::db::user_table::{create_user, get_all, UsersForm, UsersLogin, DEFAULT_PATH};
use crate::utils::cookie::{cookie_handler, create_field_cookie, handler_flash};
use crate::utils::token::{create_token, get_token};
use crate::{context, get_by_username};
use regex::Regex;
use rocket::form::Form;
use rocket::http::{CookieJar, Status};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::Template;

#[get("/home")]
pub fn home() -> Redirect {
    Redirect::to("register")
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
        ),
    )
}

/// get register with a form for the user to fill and create an account
/// if the user already login to send him a `code 405`
/// else handle the `cookie to reset back the value`
/// if the user was already trying to register and also `display the error message`
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
                    path: DEFAULT_PATH,
                    username_x,
                    password_first,
                    password_second,
                    color,
                    message
                ),
            ))
        }
    }
}

/// A post method to create a new user on the server (database)
/// need 3 inputs
///  - username_x
///  - password_x.first
///  - password_x.second
/// If user is already login send him `code 405`
/// else test for every form if is not empty
/// username don't match reserved username
/// the 2 password are a match
/// else send back to `register with an appropriate message`
/// else if all is good `create the user create the token and send him to home`
#[post("/register", data = "<form>")]
pub fn register_post(
    jar: &CookieJar<'_>,
    form: Form<UsersForm>,
) -> Result<Flash<Redirect>, Status> {
    if get_token(jar).is_ok() {
        return Result::Err(Status::MethodNotAllowed);
    }

    // closure to use when needed (if is a Err() )
    // Allow to not create cookie unnecessary
    let create_cookie = || {
        create_field_cookie(jar, "username_x", form.username_x);
        create_field_cookie(jar, "password_x.first", form.password_x.first);
        create_field_cookie(jar, "password_x.second", form.password_x.second);
    };

    // username_x not empty
    if form.username_x.is_empty() {
        create_cookie();
        return Result::Ok(Flash::error(
            Redirect::to("register"),
            "rYou need a Username",
        ));
    }

    // username_x don't match reserved username
    let regex = Regex::new("^test*").unwrap();
    let regex2 = Regex::new("#-#").unwrap();
    if form.username_x == "default.png"
        || form.username_x == "admin"
        || regex.is_match(form.username_x)
        || regex2.is_match(form.username_x)
    {
        create_cookie();
        return Result::Ok(Flash::error(
            Redirect::to("register"),
            "rThe username is reserved",
        ));
    }

    // password_x.first not empty
    if form.password_x.first.is_empty() {
        create_cookie();
        return Result::Ok(Flash::error(
            Redirect::to("register"),
            "rYou must choose a password",
        ));
    }

    // password_x.second not empty
    if form.password_x.second.is_empty() {
        create_cookie();
        return Result::Ok(Flash::error(
            Redirect::to("register"),
            "rYou have to fill the second password",
        ));
    }

    // password_x are the same
    if form.password_x.first != form.password_x.second {
        create_cookie();
        return Result::Ok(Flash::error(
            Redirect::to("register"),
            "rYour password doesn't match",
        ));
    }

    // if user not already exists
    let user_exist = get_by_username(form.username_x);
    match user_exist {
        Some(_) => {
            create_cookie();
            Result::Ok(Flash::error(
                Redirect::to("register"),
                "rThe user already exists",
            ))
        }
        None => {
            create_user(form.username_x, form.password_x.first);
            create_token(jar, form.username_x);
            Result::Ok(Flash::success(Redirect::to("home"), ""))
        }
    }
}

/// get login with a form for the user to fill and login
/// if the user already login to send him a `code 405`
/// else handle the `cookie to reset back the value`
/// if the user was already trying to login and also `display the error message`
#[get("/login")]
pub fn login(jar: &CookieJar<'_>, flash: Option<FlashMessage>) -> Result<Template, Status> {
    let (color, message) = handler_flash(flash);
    match get_token(jar) {
        Ok(_) => Err(Status::MethodNotAllowed),
        Err(_) => {
            let username_x = cookie_handler(jar, "username_x".to_string());
            let password_x = cookie_handler(jar, "password_x".to_string());
            Result::Ok(Template::render(
                "account/login",
                context!(
                    title: "Login",
                    path: DEFAULT_PATH,
                    username_x,
                    password_x,
                    color,
                    message
                ),
            ))
        }
    }
}
/// Post method to login to your account
/// if user already loggin send him `code 405`
/// return the value in the form as a cookie for the get login
/// if an error happen with an appropriate `message`
/// - `User doesn't exist`
/// - `Password incorrect`
/// if form is good login the user with a `token and send him to home`
#[post("/login", data = "<form>")]
pub fn login_put(
    jar: &CookieJar<'_>,
    form: Form<UsersLogin>,
) -> Result<Flash<Redirect>, Status> {
    let create_cookie = || {
        create_field_cookie(jar, "password_x", form.password_x);
        create_field_cookie(jar, "username_x", form.username_x);
    };

    if get_token(jar).is_ok() {
        return Result::Err(Status::MethodNotAllowed);
    }
    // if user exist
    if let Some(s) = get_by_username(form.username_x) {
        // and password match
        return if s.password == form.password_x {
            create_token(jar, form.username_x);
            Result::Ok(Flash::success(Redirect::to("home"), ""))
        } else {
            create_cookie();
            Result::Ok(Flash::error(Redirect::to("login"), "rWrong password"))
        };
    }
    create_cookie();
    Result::Ok(Flash::error(Redirect::to("login"), "rUser don't exist "))
}