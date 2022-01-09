use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;

/// Handle the flash message take the message return "" if none else the message
pub fn handler_flash(flash: Option<FlashMessage>) -> (char, String) {
    let mut val = flash
        .map(|c| c.message().to_string())
        .unwrap_or_else(|| "".to_string());

    if !val.is_empty() {
        (from_char_to_color(val.remove(0)), val)
    } else {
        (' ', val)
    }
}

/// Transform char to handling type of char
/// if r => r || g => g else return ' '
pub fn from_char_to_color(c: char) -> char {
    match c {
        'r' => 'r',
        'g' => 'g',
        _ => ' ',
    }
}
/// Return the value of the cookie and remove it from the cookiejar
/// If cookie not set return ""
pub fn cookie_handler(jar: &CookieJar, name: String) -> String {
    jar.get_pending(name.as_str()).map_or_else(
        || "".to_string(),
        |c| {
            jar.remove(Cookie::named(name));
            c.value().to_owned()
        },
    )
}

/// Add to the jar the cookie of name and value in param and also add a Duration of 3 minutes
pub fn create_field_cookie(jar: &CookieJar<'_>, name: &str, value: &str) {
    jar.add(
        Cookie::build(name.to_owned(), value.to_owned())
            .max_age(time::Duration::minutes(3))
            .finish(),
    );
}

#[cfg(test)]
mod tests {
    use crate::utils::cookie::{cookie_handler, create_field_cookie};

    //Unit test of `create_field_cookie` and `cookie handler`
    #[test]
    fn cookie() {
        use crate::rocket;
        use rocket::local::blocking::Client;
        let client = Client::tracked(rocket()).unwrap();
        let jar = &client.cookies();

        assert_eq!(
            jar.get("test_cookie"),
            None,
            "cookie should not exist at the start"
        );
        create_field_cookie(jar, "test_cookie", "12");
        assert_ne!(
            jar.get_pending("test_cookie"),
            None,
            "create_field_cookie() should create a cookie named 'test_cookie'"
        );

        assert_eq!(
            cookie_handler(jar, "test_cookie".to_string()),
            "12",
            "cookie should be a value"
        );
        assert_eq!(
            cookie_handler(jar, "test_cookie".to_string()),
            "",
            "cookie already use by 'cookie handler' so is destroy and should return default value ''"
        );
    }
}
