use crate::db::user_table::UserEntity;
use crate::get_by_username;
use rocket::http::{Cookie, CookieJar, Status};
use time::{Duration, OffsetDateTime};

pub const TOKEN: &str = "token";

/// Create the encrypted token with a Duration of 2 hours
/// With name token
/// Token is created like that : value#-#date#-#hour#-#minute
/// value is the username
/// #-# is the regex expression to separate
/// expiredate is the date when the token is expired
pub fn create_token(jar: &CookieJar<'_>, value: &str) {
    println!("Creation token ");
    let duration = OffsetDateTime::now_utc() + Duration::hours(2);
    println!(
        "{} | {} | {} ",
        duration.date().to_string(),
        duration.hour(),
        duration.minute()
    );
    jar.add_private(
        Cookie::build(
            TOKEN,
            format!(
                "{}#-#{}#-#{}#-#{}",
                value.to_owned(),
                duration.date().to_string(),
                duration.hour(),
                duration.minute(),
            ),
        ).max_age(Duration::hours(2))
        .finish(),
    )
}

/// Remove the current token
pub fn remove_token(jar: &CookieJar<'_>) {
    println!("token remove ! ");
    jar.remove_private(Cookie::named(TOKEN));
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
        let val: Vec<&str> = username.split("#-#").collect();
        if val.len() != 4 {
            println!("invalid len");
            remove_token(jar);
            return Err(Status::ExpectationFailed);
        }
        let duration = OffsetDateTime::now_utc();
        let date = duration.date().to_string();
        let hours = duration.hour().to_string();
        let min = duration.minute().to_string();
        // if date token  < current date token expired or
        // date token == current date then see hours if token hours < current hours = token expired
        // else if date and hours ==, see min
        if val[1].to_string() < date
            || val[1].to_string() == date && val[2].to_string() < hours
            || val[1].to_string() == date && val[2].to_string() == hours && val[3].to_string() < min
        {
            println!("expired token");
            remove_token(jar);
            return Err(Status::ImATeapot);
        }

        if let Some(user) = get_by_username(val[0]) {
            Ok(user)
        } else {
            println!("user don't exist");
            remove_token(jar);
            Err(Status::ExpectationFailed)
        }
    } else {
        Err(Status::Forbidden)
    }
}

/// Do a lot of operation to try to get the token if all is good will return Ok(UserEntity)
/// But if not then :
/// If user not login (token) return `error 403`
/// if login but can't find user return `error 417` and remove the token
/// if token is expire return `error 418` and remove the token
pub fn get_token(jar: &CookieJar<'_>) -> Result<UserEntity, Status> {
    get_token_spec(jar, false)
}

#[cfg(test)]
mod tests {
    use crate::get_by_username;
    use crate::utils::token::{create_token, get_token_spec, remove_token, TOKEN};
    use rocket::http::Status;

    // try a lot of scenario with the token 'get_token' 'create_token' 'remove_token' to make sur
    // that it's correctly working
    #[test]
    fn token() {
        use crate::rocket;
        use rocket::local::blocking::Client;
        let client = Client::tracked(rocket()).unwrap();
        let jar = &client.cookies();

        assert!(
            get_by_username("admin").is_some(),
            "Should have an admin account !"
        );
        create_token(jar, "admin");

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
            get("test/token").is_none(),
            "Should never exist reserved key"
        );
        create_token(jar, "test/token");

        let c = jar.get_pending(TOKEN).unwrap();
        let cookie_token = c.value();
        let token: Vec<&str> = cookie_token.split("#-#").collect();
        assert_eq!(
            token.len(),
            4,
            "token split with pattern refex should always be len 4"
        );
        assert_eq!(
            token[0], "test/token",
            "should keep the username value intact"
        );

        assert_eq!(
            get_token_spec(jar, true).err().unwrap(),
            Status::ExpectationFailed,
            "Should happen because test is not in the database"
        );

        assert_eq!(
            get_token_spec(jar, true).err().unwrap(),
            Status::Forbidden,
            "Should happen because the failed get_token should remove the cookie"
        );
    }
}