use crate::db::todo_table;
use crate::db::todo_table::{get_by_owner, CreateTodo};
use crate::utils::cookie::{cookie_handler, create_field_cookie};
use crate::utils::json::incr_to_do;
use crate::{context, get_token, handler_flash, Status};
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::Template;

/// get method to get the home of the to-do
/// return the status code if get_token send one
/// this home is just a display of the to-do of the user with some action like create delete
#[get("/home")]
pub fn home_t(jar: &CookieJar<'_>, flash: Option<FlashMessage>) -> Result<Template, Status> {
    let (form_field, message) = handler_flash(flash);
    match get_token(jar) {
        Ok(user) => {
            let todos = get_by_owner(user.id);

            Ok(Template::render(
                "todo/home",
                context!(
                    path: user.get_path(),
                    title: "Home To-Do",
                    todos,
                    form_field,
                    message
                ),
            ))
        }

        Err(status) => Err(status),
    }
}

/// get method th show the form to create a to-do
/// return the status code if get_token send one else show the template to-do create
#[get("/create")]
pub fn create_todo(jar: &CookieJar<'_>, flash: Option<FlashMessage>) -> Result<Template, Status> {
    let (form_field, message) = handler_flash(flash);
    match get_token(jar) {
        Ok(user) => {
            let title_x = cookie_handler(jar, "title_x");
            let content_x = cookie_handler(jar, "content_x");
            let date_x = cookie_handler(jar, "date_x");
            let priority_x = cookie_handler(jar, "priority_x");
            let priority_x = if priority_x.is_empty() {
                0
            } else {
                priority_x.parse::<i32>().unwrap()
            };

            Ok(Template::render(
                "todo/create",
                context!(
                path: user.get_path(),
                title: "Create To-Do",
                        title_x,
                        content_x,
                        date_x,
                        priority_x,
                    form_field,
                    message
                        ),
            ))
        }

        Err(status) => Err(status),
    }
}

/// post method to create the to-do
/// if get_token return a status code send him to the client
/// else try to see that every form is not empty and valid
/// if everything is good create the to-do incremente the to-do json and redirect to home with message
#[post("/create", data = "<form>")]
pub fn create_todo_post(
    jar: &CookieJar<'_>,
    form: Form<CreateTodo>,
) -> Result<Flash<Redirect>, Status> {
    match get_token(jar) {
        Ok(user) => {
            let create_cookie = || {
                create_field_cookie(jar, "title_x", form.title_x);
                create_field_cookie(jar, "content_x", form.content_x);
                create_field_cookie(jar, "date_x", form.date_x);
                create_field_cookie(jar, "priority_x", form.priority_x.to_string().as_str());
            };

            if form.title_x.is_empty() {
                create_cookie();
                return Ok(Flash::error(Redirect::to("create"), "tneed a title"));
            }

            if form.date_x.is_empty() {
                create_cookie();
                return Ok(Flash::error(Redirect::to("create"), "dneed a date"));
            }

            if form.date_x.len() != 10 {
                create_cookie();
                return Ok(Flash::error(Redirect::to("create"), "dinvalid date"));
            }

            if form.priority_x < 0 || form.priority_x > 10 {
                create_cookie();
                return Ok(Flash::error(Redirect::to("create"), "pinvalid value"));
            }

            if todo_table::create_todo(
                user.id,
                form.title_x,
                form.date_x,
                form.priority_x,
                form.content_x,
            ) > 0
            {
                incr_to_do();
                Ok(Flash::success(
                    Redirect::to("home"),
                    "gSuccessfully created",
                ))
            } else {
                Ok(Flash::error(
                    Redirect::to("create"),
                    "rOops. Please try again",
                ))
            }
        }
        Err(status) => Err(status),
    }
}
