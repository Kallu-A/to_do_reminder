use crate::db::user_table::UserEntity;
use dotenv::dotenv;
use lettre::message::{header, Mailbox, MessageBuilder, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rocket::serde::Serialize;
use std::env;

/// To get the code in the form
#[derive(Serialize, FromForm)]
pub struct Code<'a> {
    pub confirm_code: &'a str,
}

/// method to test if the env var allow to send email or not use to kill the server before the launch if he can't send email
pub fn verif_env() -> bool {
    let mailer = create_mailer();
    mailer.test_connection().is_ok()
}

/// Create the credentials needed to send the email
/// Create the SmtpTransport and
/// allow to easily set up the need SmtpTransport by using env var
fn create_mailer() -> SmtpTransport {
    dotenv().ok();
    let cred = Credentials::new(
        env::var("ADRESS_SMTP").expect("ADRESS_SMTP must be set"),
        env::var("PASSWORD_SMTP").expect("PASSWORD_SMTP must be set"),
    );

    SmtpTransport::relay(
        env::var("RELAY_SMTP")
            .expect("RELAY_SMTP must be set")
            .as_str(),
    )
    .expect("Error config of your SMTP env value")
    .credentials(cred)
    .build()
}

/// Create the email and use the ADRESS_SMTP var env
fn create_email(username: &str, adress: &str) -> MessageBuilder {
    dotenv().ok();
    Message::builder()
        .from(
            format!(
                "To-Do-Reminder <{}>",
                env::var("ADRESS_SMTP").expect("ADRESS_SMTP must be set")
            )
            .parse()
            .expect("Invalid ADRESS_SMTP"),
        )
        .to(format!("{} <{}>", username, adress)
            .parse()
            .unwrap_or_else(|_e| format!("user <{0}>", adress).parse::<Mailbox>().unwrap()))
}

/// Send a email to the user and return
/// a bool who tell if the send was successfull or not
pub fn send_email_code(user: &UserEntity) -> bool {
    let mailer = create_mailer();
    let mail = create_email(user.username.as_str(), user.email.as_str());
    let code = user.get_code();

    let html = format!(
        r#"<!DOCTYPE html>
                    <html lang="en">
                    <head>
                        <meta charset="UTF-8">
                        <meta name="viewport" content="width=device-width, initial-scale=1.0">
                        <title>Code to confirm your email</title>
                    </head>
                    <body>
                        <div style="text-align: center">
                            The code to confirm your email is: <b style="color: #407899; font-size: 20px">{}</b>
                        </div>
                    </body>
                    </html>"#,
        code
    );

    let mail = mail
        .subject("CODE: confirm email")
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(format!("The code to confirm your email is: {}", code)),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(html),
                ),
        )
        .expect("failed to build the email");

    mailer.send(&mail).is_ok()
}

#[cfg(test)]
mod tests {
    use crate::verif_env;

    #[test]
    fn check() {
        assert!(verif_env(), "error while verif email value");
    }
}
