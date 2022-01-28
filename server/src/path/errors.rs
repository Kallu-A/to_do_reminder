use crate::{context, get_token, DEFAULT_PATH};
use rocket::response::{Flash, Redirect};
use rocket::Request;
use rocket_dyn_templates::Template;

/// Status code who tell the user this action is for admin acccount
#[catch(401)]
pub fn unauthorize(req: &Request<'_>) -> Template {
    let path = get_path_img(req);
    Template::render(
        "error/401",
        context!(
            title: "Unauthorize action",
            path,
            uri: req.uri(),
        ),
    )
}

/// Tell that for this you need to be login
#[catch(403)]
pub fn not_login() -> Flash<Redirect> {
    Flash::success(
        Redirect::to("/account/login"),
        "rPlease log in before do this",
    )
}

/// The link or ressource was not found
#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let path = get_path_img(req);
    Template::render(
        "error/404",
        context!(
            title: "Not found",
            path,
            uri: req.uri(),
        ),
    )
}

/// This action is only for visitor
#[catch(405)]
pub fn method_not_allowed() -> Flash<Redirect> {
    Flash::success(
        Redirect::to("/account/home"),
        "rCan't do this action while you are login",
    )
}

/// Error with the token should not happen in runtime
#[catch(417)]
pub fn token_match_none() -> Template {
    Template::render(
        "error/417",
        context!(
            title: "Error Wrong Token",
            path: DEFAULT_PATH,
        ),
    )
}

/// the token is expired and you need to login again
#[catch(418)]
pub fn expired_token(req: &Request<'_>) -> Template {
    let path = get_path_img(req);
    Template::render(
        "error/418",
        context!(
            title: "Expired Token",
            path
        ),
    )
}

/// The server encounter a problem should not happen
#[catch(500)]
pub fn internal_error(req: &Request<'_>) -> Template {
    let path = get_path_img(req);
    Template::render(
        "error/500",
        context!(
            title: "Internal error",
            path,
        ),
    )
}

fn get_path_img(req: &Request<'_>) -> String {
    match get_token(req.cookies()) {
        Ok(user) => user.get_path(),
        Err(_) => DEFAULT_PATH.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use rocket::http::Status;

    #[test]
    fn check_good() {
        use crate::rocket;
        use rocket::local::blocking::Client;

        let client = Client::tracked(rocket(true)).unwrap();

        assert_ne!(
            client.get(uri!("/status/403")).dispatch().status(),
            Status::InternalServerError
        );

        assert_ne!(
            client.get(uri!("/status/404")).dispatch().status(),
            Status::InternalServerError
        );

        assert_ne!(
            client.get(uri!("/status/405")).dispatch().status(),
            Status::InternalServerError
        );

        assert_ne!(
            client.get(uri!("/status/417")).dispatch().status(),
            Status::InternalServerError
        );

        assert_ne!(
            client.get(uri!("/status/418")).dispatch().status(),
            Status::InternalServerError
        );

        assert_eq!(
            client.get(uri!("/status/500")).dispatch().status(),
            Status::InternalServerError
        );
    }
}
