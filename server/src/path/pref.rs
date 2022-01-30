use crate::db::pref_table::{get_pref_from_owner, update_pref, Mode, NewDisplay, NewMode};
use crate::{context, get_token, handler_flash, Status};
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::Template;

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
            Ok(Template::render(
                "pref/home",
                context!(
                        title: "Preferences",
                        path: user.get_path(),
                        form_field,
                        message,
                        pref,
                ),
            ))
        }

        Err(status) => Err(status),
    }
}

/// Put methode to update the value of the user pref
/// if get_token return a status show to the client
/// make sur the form is valid else redirect with error message
/// try to update the value then redirect with message
#[put("/set/display", data = "<form>")]
pub fn pref_display_put(
    jar: &CookieJar<'_>,
    form: Form<NewDisplay>,
) -> Result<Flash<Redirect>, Status> {
    match get_token(jar) {
        Ok(user) => {
            let redirect = Redirect::to("/preference/home");

            if let Some(display) = form.display_x {
                if display.is_negative() {
                    return Ok(Flash::error(redirect, "dneed a positive value"));
                }

                let mut pref = get_pref_from_owner(user.id).unwrap();
                pref.display = display;
                if update_pref(&pref) {
                    Ok(Flash::success(redirect, "gSuccessfully changed"))
                } else {
                    Ok(Flash::error(redirect, "rOops. Please try again"))
                }
            } else {
                Ok(Flash::error(redirect, "dneed a value"))
            }
        }

        Err(status) => Err(status),
    }
}

/// Put methode to update the value of the user pref
/// if get_token return a status show to the client
/// make sur the form is valid else redirect with error message
/// try to update the value then redirect with message
#[put("/set/mode", data = "<form>")]
pub fn pref_mode_put(jar: &CookieJar<'_>, form: Form<NewMode>) -> Result<Flash<Redirect>, Status> {
    match get_token(jar) {
        Ok(user) => {
            let redirect = Redirect::to("/preference/home");
            if let Some(mode) = form.mode_x {
                if Mode::from_i32(mode).is_err() {
                    return Ok(Flash::error(redirect, "munknow mode"));
                }

                let mut pref = get_pref_from_owner(user.id).unwrap();
                pref.sort = mode;
                if update_pref(&pref) {
                    Ok(Flash::success(redirect, "gSuccessfully changed"))
                } else {
                    Ok(Flash::error(redirect, "rOops. Please try again"))
                }
            } else {
                Ok(Flash::error(redirect, "mneed a mode"))
            }
        }

        Err(status) => Err(status),
    }
}
