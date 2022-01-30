use rocket::http::CookieJar;
use rocket::request::FlashMessage;
use rocket_dyn_templates::Template;
use crate::{context, get_token, handler_flash, Status};
use crate::db::pref_table::get_pref_from_owner;

/// get home page
/// here the user as accessed to all is preference and can change them
/// if get_token return a status code show it
#[get("/home")]
pub fn preference_user(jar: &CookieJar<'_>, flash: Option<FlashMessage>) -> Result<Template, Status> {
    let (form_field, message) = handler_flash(flash);
    match get_token(jar) {
        Ok(user) => {
            Ok(Template::render(
                "pref/home",
                context!(
                    title: "Preferences",
                    path: user.get_path(),
                    form_field,
                    message,
                    pref: get_pref_from_owner(user.id).unwrap()
                ),
            ))
        }

        Err(status) => Err(status),
    }
}