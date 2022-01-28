use crate::db::user_table::{
    create_user, delete_user, get_all, is_password, set_confirm_email, set_email, set_password,
    set_picture, NewEmail, UserEditPassowrd, UserRegister, UsersLogin, DEFAULT_PATH,
};
use crate::utils::cookie::{cookie_handler, create_field_cookie, handler_flash};
use crate::utils::email::{send_email_code, send_email_password, Code, ForgetPassword};
use crate::utils::json::{decr_members, incr_connexion, incr_members};
use crate::utils::token::{create_token, get_token, remove_token, TOKEN};
use crate::{context, get_by_username};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;
use rocket::form::Form;
use rocket::http::{ContentType, CookieJar, Status};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::Data;
use rocket_dyn_templates::Template;
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use std::fs;
use std::path::Path;

///The backbone of the account section
/// handler the flash message if there is one,
/// else if the user is login send him to the template `user_display` with all possible options for him
/// else if `get_token` return code 403 then redirect to `login` else display `status error from get_token`
/// if user is login and email not enable show him a message to enable email
#[get("/home")]
pub fn home(
    jar: &CookieJar<'_>,
    flash: Option<FlashMessage>,
) -> Result<Template, Result<Flash<Redirect>, Status>> {
    let (color, message) = handler_flash(flash);
    let code_confirm = cookie_handler(jar, "code_confirm".to_string());

    match get_token(jar) {
        Ok(user) => Ok(Template::render(
            "account/user_display",
            context!(
                path: user.get_path(),
                title: "Account",
                color,
                message,
                user,
                code_confirm
            ),
        )),
        Err(status) => {
            if status == Status::Forbidden {
                Err(Ok(Flash::success(
                    Redirect::to("login"),
                    format!("{}{}", color, message),
                )))
            } else {
                Err(Err(status))
            }
        }
    }
}

/// If not login or not an admin show a nice display of the user
/// if login as admin show more a state of the database with extra data like password
#[get("/users")]
pub fn users(jar: &CookieJar<'_>) -> Template {
    let users = get_all();
    let path;
    if let Ok(user) = get_token(jar) {
        path = user.get_path();
        if user.perm {
            return Template::render(
                "account/users_admin",
                context!(
                title: "Database user",
                path,
                users,
                ),
            );
        }
    } else {
        path = DEFAULT_PATH.to_string();
    }
    Template::render(
        "account/users",
        context!(
            title: "List of users",
            path,
            users,
        ),
    )
}

/// get register with a form for the user to fill and create an account
/// if the user already login to send him a `code 405`
/// else handle the `cookie to reset back the value`
/// if the user was already trying to register and also `display the error message`
#[get("/register")]
pub fn register(jar: &CookieJar<'_>, flash: Option<FlashMessage>) -> Result<Template, Status> {
    let (form_field, message) = handler_flash(flash);
    match get_token(jar) {
        Ok(_) => Err(Status::MethodNotAllowed),
        Err(_) => {
            let username_x = cookie_handler(jar, "username_x".to_string());
            let password_first = cookie_handler(jar, "password_x.first".to_string());
            let password_second = cookie_handler(jar, "password_x.second".to_string());
            let email_x = cookie_handler(jar, "email_x".to_string());
            Result::Ok(Template::render(
                "account/register",
                context!(
                    title: "Register",
                    path: DEFAULT_PATH,
                    username_x,
                    email_x,
                    password_first,
                    password_second,
                    form_field,
                    message
                ),
            ))
        }
    }
}

/// A post method to create a new user on the server (database)
/// need 3 inputs
///  - username_x
///  - password_x.first
///  - password_x.second
/// If user is already login send him `code 405`
/// else test for every form if is not empty
/// username don't match reserved username
/// the 2 password are a match
/// else send back to `register with an appropriate message`
/// else if all is good `create the user create the token and send him to home`
#[post("/register", data = "<form>")]
pub fn register_post(
    jar: &CookieJar<'_>,
    form: Form<UserRegister>,
) -> Result<Flash<Redirect>, Status> {
    if get_token(jar).is_ok() {
        return Result::Err(Status::MethodNotAllowed);
    }

    // closure to use when needed (if is a Err() )
    // Allow to not create cookie unnecessary
    let create_cookie = || {
        create_field_cookie(jar, "username_x", form.username_x);
        create_field_cookie(jar, "email_x", form.email_x);
        create_field_cookie(jar, "password_x.first", form.password_x.first);
        create_field_cookie(jar, "password_x.second", form.password_x.second);
    };

    // username_x not empty
    if form.username_x.is_empty() {
        create_cookie();
        return Result::Ok(Flash::error(Redirect::to("register"), "uneed a username"));
    }

    // username too long
    if form.username_x.len() > 15 {
        create_cookie();
        return Result::Ok(Flash::error(Redirect::to("register"), "uusername too long"));
    }

    // username_x don't match reserved username
    let regex = Regex::new("^test*").unwrap();
    if form.username_x == "default.png"
        || form.username_x == "admin"
        || regex.is_match(form.username_x)
    {
        create_cookie();
        return Result::Ok(Flash::error(
            Redirect::to("register"),
            "uusername is reserved",
        ));
    }

    if form.email_x.is_empty() {
        create_cookie();
        return Result::Ok(Flash::error(Redirect::to("register"), "eneed an email"));
    }

    let regex = Regex::new(r"^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{1,4})+$").unwrap();
    if !regex.is_match(form.email_x) {
        create_cookie();
        return Result::Ok(Flash::error(Redirect::to("register"), "einvalid email"));
    }

    // password_x.first not empty
    if form.password_x.first.is_empty() {
        create_cookie();
        return Result::Ok(Flash::error(Redirect::to("register"), "pneed a password"));
    }

    // password_x.second not empty
    if form.password_x.second.is_empty() {
        create_cookie();
        return Result::Ok(Flash::error(
            Redirect::to("register"),
            "cneed to confirm password",
        ));
    }

    // password_x are the same
    if form.password_x.first != form.password_x.second {
        create_cookie();
        return Result::Ok(Flash::error(
            Redirect::to("register"),
            "cdoesn't match the password",
        ));
    }

    // if user not already exists
    let user_exist = get_by_username(form.username_x);
    match user_exist {
        Some(_) => {
            create_cookie();
            Result::Ok(Flash::error(
                Redirect::to("register"),
                "uuser already exists",
            ))
        }
        None => {
            create_user(form.username_x, form.password_x.first, form.email_x);
            create_token(jar, &get_by_username(form.username_x).unwrap());
            incr_connexion();
            incr_members();
            send_email_code(&get_by_username(form.username_x).unwrap());
            Result::Ok(Flash::success(
                Redirect::to("home"),
                "gAccount successfully created. Please confirm your email",
            ))
        }
    }
}

/// get login with a form for the user to fill and login also a <a> to the register
/// handle flash message to nice display error in form
/// if the user already login to send him a `code 405`
/// else handle the `cookie to reset back the value`
/// if the user was already trying to login and also `display the error message`
#[get("/login")]
pub fn login(jar: &CookieJar<'_>, flash: Option<FlashMessage>) -> Result<Template, Status> {
    // input : u -> username , p -> password, ' ' -> nothing else 'r' and 'g'
    let (form_field, message) = handler_flash(flash);
    match get_token(jar) {
        Ok(_) => Err(Status::MethodNotAllowed),
        Err(_) => {
            let username_x = cookie_handler(jar, "username_x".to_string());
            let password_x = cookie_handler(jar, "password_x".to_string());

            Result::Ok(Template::render(
                "account/login",
                context!(
                    title: "Login",
                    path: DEFAULT_PATH,
                    username_x,
                    password_x,
                    form_field,
                    message
                ),
            ))
        }
    }
}

/// Post method to login to your account
/// if user already loggin send him `code 405`
/// return the value in the form as a cookie for the get login
/// if an error happen with an appropriate `message`
/// - `User doesn't exist`
/// - `Password incorrect`
/// if form is good login the user with a `token and send him to home`
#[post("/login", data = "<form>")]
pub fn login_put(jar: &CookieJar<'_>, form: Form<UsersLogin>) -> Result<Flash<Redirect>, Status> {
    let create_cookie = || {
        create_field_cookie(jar, "password_x", form.password_x);
        create_field_cookie(jar, "username_x", form.username_x);
    };

    if get_token(jar).is_ok() {
        return Result::Err(Status::MethodNotAllowed);
    }

    if form.username_x.is_empty() {
        create_cookie();
        return Result::Ok(Flash::error(
            Redirect::to("login"),
            "uplease fill the username ",
        ));
    }

    // if user exist
    if let Some(s) = get_by_username(form.username_x) {
        // and password match
        return if is_password(&s, form.password_x) {
            create_token(jar, &get_by_username(form.username_x).unwrap());
            incr_connexion();
            Result::Ok(Flash::success(Redirect::to("home"), "gYou'r logged"))
        } else {
            create_cookie();
            Result::Ok(Flash::error(Redirect::to("login"), "pwrong password"))
        };
    }
    create_cookie();
    Result::Ok(Flash::error(Redirect::to("login"), "uuser don't exist "))
}

/// PUT for trying to logout
/// if you are login then token is remove and you will be send to the success logout
/// else you have error `403`
#[put("/logout")]
pub fn home_logout(jar: &CookieJar<'_>) -> Result<Flash<Redirect>, Status> {
    if jar.get_private(TOKEN).is_some() {
        remove_token(jar);
        Result::Ok(Flash::success(Redirect::to("/"), "gSuccessfully logout"))
    } else {
        Result::Err(Status::Forbidden)
    }
}

/// DELETE for a user
/// if get_token exist,
/// try to delete a user
/// if successful redirect to `home with message` else `code 404`
/// if get_token return an error display the code status
#[delete("/delete")]
pub fn delete(jar: &CookieJar<'_>) -> Result<Flash<Redirect>, Status> {
    match get_token(jar) {
        Ok(user) => {
            if delete_user(user.username) {
                remove_token(jar);
                decr_members();
                Result::Ok(Flash::success(Redirect::to("/"), "gSuccessfully delete"))
            } else {
                Result::Err(Status::NotFound)
            }
        }
        Err(statut) => Err(statut),
    }
}

/// DELETE a user for admin
/// if user not a admin return `code 401`
/// if admin try to delete himself return `code 405`
/// try to delete the user
/// if successful redirect to `users with message` else `code 404`
/// if get_token return an error display the code status
#[delete("/delete_admin/<username>")]
pub fn delete_as_admin(jar: &CookieJar<'_>, username: String) -> Result<Flash<Redirect>, Status> {
    match get_token(jar) {
        Ok(user) => {
            if !user.perm  {
                return Err(Status::Unauthorized);
            }
            if username == user.username {
                return Err(Status::MethodNotAllowed);
            }

            if delete_user(username) {
                decr_members();
                Result::Ok(Flash::success(Redirect::to("/account/users"), "gSuccessfully delete"))
            } else {
                Result::Err(Status::NotFound)
            }
        }
        Err(statut) => Err(statut),
    }
}

/// GET edit  for a user
/// if get_token exist,
/// show form to change value
/// if get_token return an error display the code status
/// for flash :
/// 1 -> password.first | 2 -> password.second | g -> green | r -> red
#[get("/edit")]
pub fn edit(jar: &CookieJar<'_>, flash: Option<FlashMessage>) -> Result<Template, Status> {
    let (form_field, message) = handler_flash(flash);
    let password_first = cookie_handler(jar, "password_x.first".to_string());
    let password_second = cookie_handler(jar, "password_x.second".to_string());
    let email_x = cookie_handler(jar, "email_x".to_string());
    match get_token(jar) {
        Ok(user) => Ok(Template::render(
            "account/edit",
            context!(
                path: user.get_path(),
                title: "Edit Profile",
                user,
                password_first,
                password_second,
                form_field,
                message,
                email_x
            ),
        )),
        Err(e) => Err(e),
    }
}

/// A post method to create a new user on the server (database)
/// need 2 inputs
///  - password_x.first
///  - password_x.second
/// If `get_token` is err return the `code `
/// else test for every form if is not empty and password match else redirect to `edit with message`
/// else change the password
#[post("/edit", data = "<form>")]
pub fn edit_post(
    jar: &CookieJar<'_>,
    form: Form<UserEditPassowrd>,
) -> Result<Flash<Redirect>, Status> {
    let create_cookie = || {
        create_field_cookie(jar, "password_x.first", form.password_x.first);
        create_field_cookie(jar, "password_x.second", form.password_x.second);
    };

    match get_token(jar) {
        Ok(mut user) => {
            if form.password_x.first.is_empty() {
                create_cookie();
                return Ok(Flash::error(Redirect::to("edit"), "1need a password"));
            }

            if form.password_x.second.is_empty() {
                create_cookie();
                return Ok(Flash::error(
                    Redirect::to("edit"),
                    "2need to confirm password",
                ));
            }

            if form.password_x.first != form.password_x.second {
                create_cookie();
                return Ok(Flash::error(
                    Redirect::to("edit"),
                    "2doesn't match the password",
                ));
            }

            if set_password(user.username.as_str(), form.password_x.first) {
                remove_token(jar);
                user.password = form.password_x.first.to_string();
                create_token(jar, &user);
                Ok(Flash::success(Redirect::to("edit"), "gPassword changed"))
            } else {
                Ok(Flash::error(
                    Redirect::to("edit"),
                    "rOops. Please try again",
                ))
            }
        }
        Err(e) => Err(e),
    }
}

/// A post method to remove the picture of the user
/// first look if id is the token.id if not return `status 405`
/// else if user already don't have a picture redirect him to `edit with message`
/// else remove and show him successful message
#[delete("/edit/remove_picture/<id>")]
pub fn remove_picture(
    jar: &CookieJar<'_>,
    id: i32,
) -> Result<Flash<Redirect>, Status> {
    match get_token(jar) {
        Ok(mut user) => {
            let redirect =
            if user.perm {
                Redirect::to("/account/users")
            } else {
                if id != user.id {
                    return Err(Status::MethodNotAllowed);
                }
                Redirect::to("/account/edit")
            };

            if !user.picture {
                return Ok(Flash::error(redirect, "rYou already don't have a picture"));
            }

            let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "static/image/profil");
            let pa = Path::new(root).join(user.id.to_string().as_str());

            if fs::remove_file(pa).is_ok() {
                user.picture = false;
                remove_token(jar);
                create_token(jar, &user);
                set_picture(user.username.as_str(), false);
                Ok(Flash::error(redirect, "gSuccessfully remove"))
            } else {
                Ok(Flash::error(redirect, "rOops. Internal error, pleasy try again"))
            }


        }
        Err(e) => Err(e),
    }
}

/// If `get_token` return an error display the `status code`
/// If picture is too large `size limite of : 1MB` to avoid attack with large image then send an `appropriate message`
/// else try to save the picture then redirect to `account with an appropriate message`
#[post("/set/picture", data = "<data>")]
pub async fn upload_picture(
    jar: &CookieJar<'_>,
    data: Data<'_>,
    content_type: &ContentType,
) -> Result<Flash<Redirect>, Status> {
    match get_token(jar) {
        Ok(user) => {
            let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
                MultipartFormDataField::file("picture")
                    .size_limit(1024 * 1024)
                    .content_type_by_string(Some(mime::IMAGE_STAR))
                    .unwrap(),
            ]);
            if let Ok(multipart_form_data) =
                MultipartFormData::parse(content_type, data, options).await
            {
                if let Some(picture) = multipart_form_data.files.get("picture") {
                    let picture = &picture[0];
                    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "static/image/profil");
                    let pa = Path::new(root).join(user.id.to_string().as_str());

                    let path = &picture.path.to_owned();

                    if fs::copy(path, pa).is_ok() {
                        let username = user.username.clone();
                        set_picture(user.username.as_str(), true);
                        // needed to get the new value in the token
                        remove_token(jar);
                        create_token(jar, &get_by_username(username.as_str()).unwrap());
                        Ok(Flash::success(
                            Redirect::to("/account/edit"),
                            "gImage successfully change !",
                        ))
                    } else {
                        Ok(Flash::error(
                            Redirect::to("/account/edit"),
                            "rError reading the file !",
                        ))
                    }
                } else {
                    Ok(Flash::error(
                        Redirect::to("/account/edit"),
                        "rPlease select a new image !",
                    ))
                }
            } else {
                Ok(Flash::error(
                    Redirect::to("/account/edit"),
                    "rPicture too large !",
                ))
            }
        }
        Err(status) => Err(status),
    }
}

/// Allow to send the code to confirm the email
/// Try the token return error of `get_token`
/// if user already have is email enable redirect to home
/// else send him the code
#[put("/send_code")]
pub fn send_code(jar: &CookieJar<'_>) -> Result<Flash<Redirect>, Status> {
    match get_token(jar) {
        Ok(user) => {
            if user.confirm_email {
                return Ok(Flash::error(
                    Redirect::to("home"),
                    "rEmail is already confirmed",
                ));
            }
            if send_email_code(&user) {
                Ok(Flash::success(Redirect::to("home"), "gCode send"))
            } else {
                Ok(Flash::error(
                    Redirect::to("home"),
                    "rError while sending the code",
                ))
            }
        }
        Err(e) => Err(e),
    }
}

/// Try a code to see if the email is good
/// Try the token return error of `get_token`
/// if user already enable is email, send him to `home`
/// else compare the form and the code and redirect him to `home`
/// with the appropriate message
#[post("/confirm", data = "<code>")]
pub fn confirm_code(jar: &CookieJar<'_>, code: Form<Code>) -> Result<Flash<Redirect>, Status> {
    let create_cookie = || {
        create_field_cookie(jar, "code_confirm", code.confirm_code);
    };
    match get_token(jar) {
        Ok(mut user) => {
            if user.confirm_email {
                create_cookie();
                return Ok(Flash::error(
                    Redirect::to("home"),
                    "rEmail is already confirmed",
                ));
            }
            println!("{}, {}", code.confirm_code, user.get_code());
            if code.confirm_code == user.get_code().as_str() {
                remove_token(jar);
                let confirm = set_confirm_email(user.username.as_str());
                user.confirm_email = confirm;
                create_token(jar, &user);
                Ok(Flash::success(Redirect::to("home"), "gEmail confirm"))
            } else {
                create_cookie();
                Ok(Flash::error(Redirect::to("home"), "rWrong code"))
            }
        }
        Err(e) => Err(e),
    }
}

/// Form to send a new password to the email and username
/// if user is already login, redirect him to `login`
/// show the form and the message
#[get("/code_password")]
pub fn form_password_change(
    jar: &CookieJar<'_>,
    flash: Option<FlashMessage>,
) -> Result<Template, Flash<Redirect>> {
    let (form_field, message) = handler_flash(flash);
    let username_x = cookie_handler(jar, "username_x".to_string());
    let email_x = cookie_handler(jar, "email_x".to_string());

    if get_token(jar).is_ok() {
        Err(Flash::error(
            Redirect::to("login"),
            "rYou can change your password here",
        ))
    } else {
        Ok(Template::render(
            "account/forgot_password",
            context!(
                title: "Forgot Password",
                path: DEFAULT_PATH,
                form_field,
                message,
                username_x,
                email_x
            ),
        ))
    }
}

/// Change the password of the user and send to the email associate
/// if user is already login return `code 405`
/// if email not valid regex show him the error
/// else try to find the user with the email and the username, if not found show him a `message of error`
/// if email is not enable show him a `message of error`
/// else change the password to a random code and send the code to the email
#[put("/code_password", data = "<data>")]
pub fn password_code(
    jar: &CookieJar<'_>,
    data: Form<ForgetPassword>,
) -> Result<Flash<Redirect>, Status> {
    if get_token(jar).is_ok() {
        return Err(Status::MethodNotAllowed);
    }

    let create_cookie = || {
        create_field_cookie(jar, "username_x", data.username);
        create_field_cookie(jar, "email_x", data.email);
    };

    let regex = Regex::new(r"^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{1,4})+$").unwrap();
    if !regex.is_match(data.email) {
        create_cookie();
        return Ok(Flash::error(
            Redirect::to("code_password"),
            "einvalid email",
        ));
    }

    if let Some(user) = get_by_username(data.username) {
        if user.email != data.email {
            create_cookie();
            return Ok(Flash::error(
                Redirect::to("code_password"),
                "enot the email you're using",
            ));
        }

        if !user.confirm_email {
            create_cookie();
            Ok(Flash::error(
                Redirect::to("code_password"),
                "rYour email is not verified",
            ))
        } else {
            let new_password: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect();

            set_password(user.username.as_str(), new_password.as_str());
            if !send_email_password(&user, new_password.as_str()) {
                create_cookie();
                return Ok(Flash::error(
                    Redirect::to("code_password"),
                    "rError unable to send the email",
                ));
            }

            Ok(Flash::error(
                Redirect::to("code_password"),
                "gNew password sends please login with it",
            ))
        }
    } else {
        create_cookie();
        Ok(Flash::error(
            Redirect::to("code_password"),
            "uunable to find your account",
        ))
    }
}

/// Change the email ans set the new email to confirm_email = false
/// if user not login `return the error get by get_token()`
/// if field email not fill or invalid email redirect to form with appropriate message
/// else change the email send the new code of confirm and show the success message ahd change the token
#[put("/new_email", data = "<data>")]
pub fn new_email(jar: &CookieJar<'_>, data: Form<NewEmail>) -> Result<Flash<Redirect>, Status> {
    match get_token(jar) {
        Ok(mut user) => {
            let create_cookie = || {
                create_field_cookie(jar, "email_x", data.email_x);
            };

            if data.email_x.is_empty() {
                create_cookie();
                return Result::Ok(Flash::error(Redirect::to("edit"), "efill a new email"));
            }

            let regex = Regex::new(r"^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{1,4})+$").unwrap();
            if !regex.is_match(data.email_x) {
                create_cookie();
                return Result::Ok(Flash::error(Redirect::to("edit"), "einvalid email"));
            }

            if data.email_x == user.email {
                create_cookie();
                return Result::Ok(Flash::error(
                    Redirect::to("edit"),
                    "eit's already your email",
                ));
            }

            if !set_email(user.username.as_str(), data.email_x) {
                create_cookie();
                return Result::Ok(Flash::error(
                    Redirect::to("edit"),
                    "rError saving the new email",
                ));
            }

            user.email = data.email_x.to_string();
            user.confirm_email = false;
            remove_token(jar);
            create_token(jar, &user);
            if !send_email_code(&user) {
                create_cookie();
                return Result::Ok(Flash::error(
                    Redirect::to("edit"),
                    "rCan't send the confirm code",
                ));
            }

            Result::Ok(Flash::error(
                Redirect::to("edit"),
                "gEmail successfully change, please look your email",
            ))
        }
        Err(status) => Err(status),
    }
}

#[cfg(test)]
mod tests {
    use crate::rocket;
    use rocket::http::Status;

    #[test]
    fn test_account() {
        use crate::rocket;
        use rocket::local::blocking::Client;
        let client = Client::tracked(rocket(true)).unwrap();

        assert_eq!(
            client.get(uri!("/account/home")).dispatch().status(),
            Status::SeeOther
        );
        assert_eq!(
            client.get(uri!("/account/users")).dispatch().status(),
            Status::Ok
        );
        assert_eq!(
            client.get(uri!("/account/register")).dispatch().status(),
            Status::Ok
        );
        assert_eq!(
            client.get(uri!("/account/login")).dispatch().status(),
            Status::Ok
        );
        assert_eq!(
            client.get(uri!("/account/edit")).dispatch().status(),
            Status::SeeOther
        );
        assert_eq!(
            client
                .get(uri!("/account/code_password"))
                .dispatch()
                .status(),
            Status::Ok
        );
    }
}
