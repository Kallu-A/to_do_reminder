use rocket::form::Form;
use crate::db::pref_table::{get_pref_from_owner, Mode, NewDisplay, NewMode, update_pref};
use crate::{context, get_token, handler_flash, Status};
use rocket::http::CookieJar;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::Template;
use crate::utils::cookie::{cookie_handler, create_field_cookie};

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
            let pref = get_pref_from_owner(user.id).unwrap();
            let display_x = cookie_handler(jar, "display_x");
            let mode_x = cookie_handler(jar, "mode_x");

            Ok(Template::render(
                "pref/home",
                context!(
                    title: "Preferences",
                    path: user.get_path(),
                    form_field,
                    message,
                    pref,
                    display_x,
                    mode_x
            ),
            ))
        },

        Err(status) => Err(status),
    }
}


/// Put methode to update the value of the user pref
/// if get_token return a status show to the client
/// make sur the form is valid else redirect with error message
/// try to update the value then redirect with message
#[put("/set/display",  data = "<form>")]
pub fn pref_display_put(
    jar: &CookieJar<'_>,
    form: Form<NewDisplay>,
) -> Result<Flash<Redirect>, Status> {
    match get_token(jar) {
        Ok(user) => {
            let create_cookie = || {
                let val = form.display_x.unwrap_or_else(|| -1).to_string();
                let val = if val == "-1" { "" } else { val.as_str() };
                create_field_cookie(jar, "display_x", val);
            };
            let redirect = Redirect::to("/preference/home");

            if let Some(display) = form.display_x {
                if display.is_negative() {
                    create_cookie();
                    return Ok(Flash::error(redirect, "dneed a positive value"));
                }

                let mut pref = get_pref_from_owner(user.id).unwrap();
                pref.display = display;
                if update_pref(&pref) {
                    Ok(Flash::success(redirect, "gSuccessfully changed"))
                } else {
                    create_cookie();
                    Ok(Flash::error(redirect, "rOops. Please try again"))
                }
            } else {
                create_cookie();
                return Ok(Flash::error(redirect, "dneed a value"));
            }

        }

        Err(status) => Err(status)
    }
}

/// Put methode to update the value of the user pref
/// if get_token return a status show to the client
/// make sur the form is valid else redirect with error message
/// try to update the value then redirect with message
#[put("/set/mode",  data = "<form>")]
pub fn pref_mode_put(
    jar: &CookieJar<'_>,
    form: Form<NewMode>,
) -> Result<Flash<Redirect>, Status> {
    match get_token(jar) {
        Ok(user) => {
            let create_cookie = || {
                let val = form.mode_x.unwrap_or_else(|| -1).to_string();
                let val = if val == "-1" { "" } else { val.as_str() };
                create_field_cookie(jar, "mode_x", val);
            };
            let redirect = Redirect::to("/preference/home");
            if let Some(mode) = form.mode_x {
                if Mode::from_i32(mode).is_err() {
                    create_cookie();
                    return Ok(Flash::error(redirect, "munknow mode"));
                }

                let mut pref = get_pref_from_owner(user.id).unwrap();
                pref.sort = mode;
                if update_pref(&pref) {
                    Ok(Flash::success(redirect, "gSuccessfully changed"))
                } else {
                    create_cookie();
                    Ok(Flash::error(redirect, "rOops. Please try again"))
                }
            } else {
                create_cookie();
                return Ok(Flash::error(redirect, "mneed a mode"));
            }

        }

        Err(status) => Err(status)
    }
}