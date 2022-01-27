use crate::db::user_table::UserEntity;
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use rocket::http::{Cookie, CookieJar, Status};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::env;
use time::Duration;

pub const TOKEN: &str = "token";
pub const TOKEN_DATE: &str = "token-set";

/// Token structure to serialize deserialize
#[derive(Default, Deserialize, Serialize)]
struct TokenEntity {
    id: i32,
    username: String,
    password: String,
    email: String,
    confirm_email: bool,
    picture: bool,
    perm: bool,
}

/// Create the encrypted token with a Duration of 2 hours for expired token
/// set a TOKEN-SET to handle when a token is expired who's limited in time of 12 hours
/// With name tokento_do_reminder
/// return bool if create or not
pub fn create_token(jar: &CookieJar<'_>, user: &UserEntity) -> bool {
    println!("Creation token ");
    let val = new_token(user);
    if val.is_err() {
        return false;
    }
    let val = val.unwrap();

    jar.add_private(
        Cookie::build(TOKEN, val)
            .max_age(Duration::hours(2))
            .finish(),
    );
    jar.add_private(
        Cookie::build(TOKEN_DATE, "")
            .max_age(Duration::hours(12))
            .finish(),
    );
    true
}

/// Work as explain in function 'get_token' but with the test bool if is true will
/// get pending cookie for test purpose
fn get_token_spec(jar: &CookieJar<'_>, test: bool) -> Result<UserEntity, Status> {
    if let Some(username) = if test {
        jar.get_pending(TOKEN)
    } else {
        jar.get_private(TOKEN)
    }
    .map(|c| c.value().to_string())
    {
        let token = login_token(username.as_str());
        if token.is_err() {
            remove_token(jar);
            return Err(Status::InternalServerError);
        }
        let token = token.unwrap();

        Ok(UserEntity {
            id: token.id,
            username: token.username,
            password: token.password,
            perm: token.perm,
            picture: token.picture,
            email: token.email,
            confirm_email: token.confirm_email,
        })
    } else if if test {
        jar.get_pending(TOKEN_DATE)
    } else {
        jar.get_private(TOKEN_DATE)
    }
    .is_some()
    {
        remove_token(jar);
        Err(Status::ImATeapot)
    } else {
        Err(Status::Forbidden)
    }
}

/// Remove the current token
pub fn remove_token(jar: &CookieJar<'_>) {
    println!("token remove ! ");
    jar.remove_private(Cookie::named(TOKEN));
    jar.remove_private(Cookie::named(TOKEN_DATE));
}

/// Do a lot of operation to try to get the token if all is good will return Ok(UserEntity)
/// But if not then :
/// If user not login (token) return `error 403`
/// if error while get the token return `error 500` and remove the token
pub fn get_token(jar: &CookieJar<'_>) -> Result<UserEntity, Status> {
    get_token_spec(jar, false)
}

/// Create a token and return a book if successful or not
fn new_token(user_x: &UserEntity) -> Result<String, String> {
    let header: Header = Default::default();
    // create the serialize struct
    let claims = TokenEntity {
        id: user_x.id,
        username: user_x.username.clone(),
        password: user_x.password.clone(),
        email: user_x.email.clone(),
        confirm_email: user_x.confirm_email,
        picture: user_x.picture,
        perm: user_x.perm,
    };

    let unsigned_token = Token::new(header, claims);
    dotenv().ok();
    let key: Hmac<Sha256> = Hmac::new_from_slice(
        env::var("TOKEN_KEY")
            .expect("TOKEN_KEY must be set")
            .as_bytes(),
    )
    .map_err(|_e| "Unable to create the token")?;
    let signed_token = unsigned_token
        .sign_with_key(&key)
        .map_err(|_e| "Invalid key")?;

    Ok(signed_token.into())
}

/// Try to get the token return the structure if it's a success, else return status code
fn login_token(token: &str) -> Result<TokenEntity, &'static str> {
    dotenv().ok();
    let key: Hmac<Sha256> = Hmac::new_from_slice(
        env::var("TOKEN_KEY")
            .expect("TOKEN_KEY must be set")
            .as_bytes(),
    )
    .map_err(|_e| "Invalid key")?;

    let token: Token<Header, TokenEntity, _> =
        VerifyWithKey::verify_with_key(token, &key).map_err(|_e| "Verification failed")?;

    let (_, claims) = token.into();
    Ok(claims)
}

#[cfg(test)]
mod tests {
    use crate::db::user_table::UserEntity;
    use crate::get_by_username;
    use crate::utils::token::{create_token, get_token_spec, login_token, new_token, remove_token};
    use rocket::http::Status;

    // try a lot of scenario with the token 'get_token' 'create_token' 'remove_token' to make sur
    // that it's correctly working
    #[test]
    fn token() {
        use crate::rocket;
        use rocket::local::blocking::Client;
        let client = Client::tracked(rocket(true)).unwrap();
        let jar = &client.cookies();

        let admin = UserEntity {
            id: 0,
            username: "admin".to_string(),
            password: "1".to_string(),
            perm: false,
            picture: false,
            email: "test@gmail.com".to_string(),
            confirm_email: true,
        };

        create_token(jar, &admin);

        assert!(
            get_token_spec(jar, true).is_ok(),
            "Should exist because was created just before"
        );
        remove_token(jar);
        assert_eq!(
            get_token_spec(jar, true).err().unwrap(),
            Status::Forbidden,
            "Should not exist because was remove"
        );

        assert!(
            get_by_username("test/token").is_none(),
            "Should never exist reserved key"
        );
    }

    // assure the data in the token are correctly generated
    #[test]
    fn generate_token() {
        let user = UserEntity {
            id: 10,
            username: "user".to_string(),
            password: "pass".to_string(),
            perm: true,
            picture: false,
            email: "test@gmail.com".to_string(),
            confirm_email: true,
        };
        let token = new_token(&user);
        assert!(token.is_ok());
        let token = token.unwrap();

        let res = login_token(token.as_str());
        assert!(res.is_ok());
        assert!(login_token("wrong token").is_err());
        let token = res.unwrap();
        assert_eq!(token.username, "user");
        assert_eq!(token.password, "pass");
        assert_eq!(token.perm, true);
        assert_eq!(token.picture, false);
        assert_eq!(token.id, 10);
        assert_eq!(token.confirm_email, true);
        assert_eq!(token.email, "test@gmail.com");
    }
}
