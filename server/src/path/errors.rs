use crate::{context, DEFAULT_PATH, get_token};
use rocket::Request;
use rocket_dyn_templates::Template;

#[catch(403)]
pub fn not_login(req: &Request<'_>) -> Template {
    Template::render(
        "error/403",
        context!(
            title: "ERROR 403",
            path: DEFAULT_PATH,
            uri: req.uri(),
        ),
    )
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let path = get_path_img(req);
    Template::render(
        "error/404",
        context!(
            title: "ERROR 404",
            path,
            uri: req.uri(),
        ),
    )
}

#[catch(405)]
pub fn method_not_allowed(req: &Request<'_>) -> Template {
    let path = get_path_img(req);
    Template::render(
        "error/405",
        context!(
            title: "ERROR 405",
            path,
            uri: req.uri(),
        ),
    )
}

#[catch(417)]
pub fn token_match_none() -> Template {
    Template::render(
        "error/417",
        context!(
            title: "ERROR 417",
            path: DEFAULT_PATH,
        ),
    )
}

#[catch(418)]
pub fn expired_token(req: &Request<'_>) -> Template {
    let path = get_path_img(req);
    Template::render(
        "error/418",
        context!(
            title: "ERROR 418",
            path
        ),
    )
}

#[catch(500)]
pub fn internal_error(req: &Request<'_>) -> Template {
    let path = get_path_img(req);
    Template::render(
        "error/500",
        context!(
            title: "ERROR 500",
            path,
        ),
    )
}


fn get_path_img(req: &Request<'_>) -> String {
    match get_token(req.cookies()) {
        Ok(user) => { user.get_path() }
        Err(_) => { DEFAULT_PATH.to_string() }
    }
}