use crate::db::pref_table::get_pref_from_owner;
use crate::{context, get_token, handler_flash, Status};
use rocket::http::CookieJar;
use rocket::request::FlashMessage;
use rocket_dyn_templates::Template;
use crate::utils::cookie::cookie_handler;

/// get home page
/// here the user as accessed to all is preference and can change them
/// if get_token return a status code show it
#[get("/home")]
pub fn preference_user(
    jar: &CookieJar<'_>,
    flash: Option<FlashMessage>,
) -> Result<Template, Status> {
    let (form_field, message) = handler_flash(flash);
    match get_token(jar) {
        Ok(user) => {
            let display_x = cookie_handler(jar, "display_x");
            let mode_x = cookie_handler(jar, "mode_x");

            Ok(Template::render(
                "pref/home",
                context!(
                    title: "Preferences",
                    path: user.get_path(),
                    form_field,
                    message,
                    pref: get_pref_from_owner(user.id).unwrap(),
                    display_x,
                    mode_x
            ),
            ))
        },

        Err(status) => Err(status),
    }
}
