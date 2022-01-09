use rocket::Request;
use rocket_dyn_templates::Template;
use crate::{context, DEFAULT_PATH};

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