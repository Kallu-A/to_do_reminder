use crate::{context, get_token, DEFAULT_PATH};
use rocket::response::{Flash, Redirect};
use rocket::Request;
use rocket_dyn_templates::Template;

#[catch(403)]
pub fn not_login() -> Flash<Redirect> {
    Flash::success(
        Redirect::to("/account/login"),
        "rPlease log in before do this",
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
pub fn method_not_allowed() -> Flash<Redirect> {
    Flash::success(
        Redirect::to("/account/home"),
        "rCan't do this action while you are login",
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
