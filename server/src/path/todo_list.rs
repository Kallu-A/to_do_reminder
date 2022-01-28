use crate::{context, get_token, Status};
use rocket::http::CookieJar;
use rocket_dyn_templates::Template;

/// get method to get the home of the to-do
/// return the status code if get_token send one
#[get("/home")]
pub fn home_t(jar: &CookieJar<'_>) -> Result<Template, Status> {
    match get_token(jar) {
        Ok(user) => Ok(Template::render(
            "todo/home",
            context!(
            path: user.get_path(),
            title: "Home To-Do"
                    ),
        )),

        Err(status) => Err(status),
    }
}

/// get method th show the form to create a to-do
/// /// return the status code if get_token send one
#[get("/create")]
pub fn create_todo(jar: &CookieJar<'_>) -> Result<Template, Status> {
    match get_token(jar) {
        Ok(user) => Ok(Template::render(
            "todo/create",
            context!(
            path: user.get_path(),
            title: "Create To-Do"
                    ),
        )),

        Err(status) => Err(status),
    }
}
