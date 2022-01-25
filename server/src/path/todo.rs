use crate::{context, DEFAULT_PATH};
use rocket_dyn_templates::Template;

/// test
#[get("/home")]
pub fn test() -> Template {
    Template::render(
        "todo/test",
        context!(
            path: DEFAULT_PATH,
            title: "To-Do test",
        ),
    )
}
