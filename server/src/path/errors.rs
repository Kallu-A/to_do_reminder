use crate::{context, DEFAULT_PATH};
use rocket::Request;
use rocket_dyn_templates::Template;

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        context!(
            title: "ERROR 404",
            path: DEFAULT_PATH,
            uri: req.uri(),
        ),
    )
}

#[catch(405)]
pub fn method_not_allowed(req: &Request<'_>) -> Template {
    Template::render(
        "error/405",
        context!(
            title: "ERROR 405",
            path: DEFAULT_PATH,
            uri: req.uri(),
        ),
    )
}