use dotenv::dotenv;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::transport::smtp::Error;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

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
fn create_email(username: &str, adress: &str, suject: String, body: String) -> Message {
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
            .expect("Wrong adress"))
        .subject(suject)
        .body(body)
        .expect("error while creating the email")
}

pub fn send_email() -> Result<Response, Error> {
    let email = create_email(
        "",
        "",
        "".to_string(),
        "".to_string(),
    );
    let mailer = create_mailer();

    mailer.send(&email)
}

#[cfg(test)]
mod tests {
    use crate::verif_env;

    #[test]
    fn check() {
        assert!(verif_env(), "error while verif email value");
    }
}
