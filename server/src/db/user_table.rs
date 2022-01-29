use diesel::prelude::*;
use regex::Regex;
use rocket::serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, fs};

use crate::db::handler;
use crate::db::handler::establish_connection;
use crate::db::pref_table::{create_pref, delete_pref};
use crate::db::todo_table::delete_by_owner;
use crate::schema::user;
use crate::schema::user::dsl::*;
use dotenv::dotenv;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier};
use pbkdf2::Pbkdf2;

pub const DEFAULT_PATH: &str = "/static/image/profil/default.png";

/// Struct who represent a user
#[derive(Queryable, Serialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "user"]
pub struct UserEntity {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub perm: bool,
    pub picture: bool,
    pub email: String,
    pub confirm_email: bool,
}

impl UserEntity {
    /// Return the path of the file of the profile picture if has it or else default.png
    pub fn get_path(&self) -> String {
        let mut path = "/static/image/profil/".to_string();
        // set picture to the user if is has one else default
        if self.picture {
            path.push_str(self.id.to_string().as_str());
        } else {
            path.push_str("default.png");
        }
        path
    }

    /// Generate a unique code for every user (use to confirm email)
    pub fn get_code(&self) -> String {
        let mut code = "".to_string();
        code.push_str(self.username.len().to_string().as_str());
        code.push_str(self.id.to_string().drain(0..1).as_str());
        code.push_str(
            self.username
                .chars()
                .map(|c| (c.to_ascii_lowercase() as u32).to_string())
                .collect::<String>()
                .as_str(),
        );

        code.truncate(10);
        code
    }
}
/// Struct to put a new user on the table User
#[derive(Insertable)]
#[table_name = "user"]
struct NewUserEntity<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub perm: bool,
    pub picture: bool,
    pub email: &'a str,
    pub confirm_email: bool,
}

/// Struct used to create a new User by form with 2 password to make user confirm is password
#[derive(Debug, FromForm, Serialize)]
pub struct UserRegister<'a> {
    pub username_x: &'a str,
    pub email_x: &'a str,
    pub password_x: Password<'a>,
}

#[derive(FromForm, Serialize)]
pub struct UserEditPassowrd<'a> {
    pub username_x: &'a str,
    pub password_x: Password<'a>,
}

/// Struct to let the user try to login
#[derive(FromForm, Deserialize)]
pub struct UsersLogin<'a> {
    pub username_x: &'a str,
    pub password_x: &'a str,
}

/// Needed in the creation to ensure their is no typo in the password
#[derive(Debug, FromForm, Serialize)]
pub struct Password<'a> {
    pub first: &'a str,
    pub second: &'a str,
}

#[derive(FromForm, Serialize)]
pub struct NewEmail<'a> {
    pub email_x: &'a str,
}

/// Return the user with the username in param or None if he doesn't exist
pub fn get_by_username(username_find: &str) -> Option<UserEntity> {
    let conn = &mut handler::establish_connection();
    match user
        .filter(username.eq(username_find))
        .load::<UserEntity>(conn)
    {
        Ok(mut res) => {
            if !res.is_empty() {
                res.drain(0..1).next()
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// Return the user with the id in param or None if he doesn't exist
pub fn get_by_id(id_find: i32) -> Option<UserEntity> {
    let conn = &mut handler::establish_connection();
    match user.filter(id.eq(id_find)).load::<UserEntity>(conn) {
        Ok(mut res) => {
            if !res.is_empty() {
                res.drain(0..1).next()
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// Return all the User on the Table `user`
pub fn get_all() -> Vec<UserEntity> {
    let connection = &mut handler::establish_connection();
    user.load::<UserEntity>(connection)
        .expect("Error loading user")
}

/// Try to delete the username in args and return true if the delete was successful else false
pub fn delete_user(username_delete: String) -> bool {
    let connection = &mut handler::establish_connection();
    let user_x = get_by_username(username_delete.as_str()).unwrap();
    let num_deleted = diesel::delete(user.filter(username.eq(username_delete.clone())))
        .execute(connection)
        .expect("Error deleting user");

    println!("-- Deleted {} user", num_deleted);
    if num_deleted > 0 {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "static/image/profil");
        let pa = Path::new(root).join(&username_delete);
        if fs::remove_file(pa).is_ok() {
            println!("picture of {} deleted", username_delete);
        }
        delete_by_owner(user_x.id);
        delete_pref(user_x.id);

        true
    } else {
        false
    }
}

/// Create a new user in the table User
/// if username is reserved key 'default.png' return 0 and don't create the user
/// If the username match '^test*' Regex and admin_x is not true also return 0 and don't create the user
/// if admin_x is true create the user even if the pattern match '^test*`
/// username: &str a primary key
/// password will be hashed
/// email
/// admin for the permission
/// the confirm_email value will be the same as admin
pub fn create_user_perm(username_x: &str, password_x: &str, email_x: &str, admin_x: bool) -> usize {
    let conn = &handler::establish_connection();

    let regex = Regex::new("^test*").unwrap();
    if username_x == "default.png" || regex.is_match(username_x) && !admin_x {
        return 0;
    }

    dotenv().ok();
    let key = env::var("SECRET_KEY").expect("Secret_key must be set");
    let password_hash = Pbkdf2
        .hash_password_simple(password_x.as_bytes(), key.as_str())
        .unwrap()
        .to_string();

    let new_user = NewUserEntity {
        username: username_x,
        password: password_hash.as_str(),
        email: email_x,
        perm: admin_x,
        picture: false,
        confirm_email: admin_x,
    };

    let num = diesel::insert_into(user::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");

    if let Some(user_x) = get_by_username(username_x) {
        create_pref(user_x.id);
    }

    num
}

/// Allow to also create a user,
/// but without the need of specifying a boolean
/// value for perm default value will be false
pub fn create_user(username_a: &str, password_a: &str, email_x: &str) -> usize {
    create_user_perm(username_a, password_a, email_x, false)
}

/// Allow to change the value of a user picture book to change if he as a profile picture or not
pub fn set_picture(user_x: &str, pic: bool) -> bool {
    if let Some(mut us) = get_by_username(user_x) {
        let con = &mut establish_connection();
        us.picture = pic;
        us.save_changes::<UserEntity>(con).is_ok()
    } else {
        false
    }
}

/// Try to change the password of user_x
pub fn set_password(user_x: &str, password_x: &str) -> bool {
    if let Some(mut us) = get_by_username(user_x) {
        let con = &mut establish_connection();
        dotenv().ok();
        let key = env::var("SECRET_KEY").expect("Secret_key must be set");
        let password_hash = Pbkdf2
            .hash_password_simple(password_x.as_bytes(), key.as_str())
            .unwrap()
            .to_string();
        us.password = password_hash;
        us.save_changes::<UserEntity>(con).is_ok()
    } else {
        false
    }
}

/// allow change of a new email and set active to false
pub fn set_email(user_x: &str, email_x: &str) -> bool {
    if let Some(mut us) = get_by_username(user_x) {
        let con = &mut establish_connection();
        dotenv().ok();
        us.email = email_x.to_string();
        us.confirm_email = false;
        us.save_changes::<UserEntity>(con).is_ok()
    } else {
        false
    }
}

/// Try to put true to the confirm_email field of the `user_x`
pub fn set_confirm_email(user_x: &str) -> bool {
    if let Some(mut us) = get_by_username(user_x) {
        let con = &mut establish_connection();
        dotenv().ok();
        us.confirm_email = true;
        us.save_changes::<UserEntity>(con).is_ok()
    } else {
        false
    }
}

/// Test if password_x not hashed is the same as the password of the user
pub fn is_password(us: &UserEntity, password_x: &str) -> bool {
    let parsed_hash = PasswordHash::new(us.password.as_str()).unwrap();
    Pbkdf2
        .verify_password(password_x.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use crate::db::user_table::{
        delete_user, get_by_id, is_password, set_confirm_email, set_email, set_password,
        set_picture,
    };
    use crate::{create_user_perm, get_by_username};
    use std::panic;

    #[test]
    fn check() {
        panic::set_hook(Box::new(|err| {
            delete_user("test/user_table".to_string());
            println!("\n{}", err.to_string());
            println!("{}", err.location().unwrap().to_string());
        }));

        assert!(
            get_by_username("test/user_table").is_none(),
            "reserved username"
        );
        assert_eq!(
            create_user_perm("test/user_table", "1", "yo@gmail.com", true),
            1
        );
        let userx = get_by_username("test/user_table");
        assert!(userx.is_some(), "just create");
        assert!(get_by_id(-1).is_none());
        let userx = userx.unwrap();
        assert!(is_password(&userx, "1"));
        assert_eq!(userx.email, "yo@gmail.com");
        assert_eq!(userx.confirm_email, true);
        assert!(!is_password(&userx, "4"));
        assert_eq!(userx.picture, false, "default value");
        assert_eq!(set_password(userx.username.as_str(), "5"), true);
        assert_eq!(set_password("test/user_table2", "2"), false);
        set_email(userx.username.as_str(), "ya@gmail.com");
        set_picture(userx.username.as_str(), true);
        let userx = get_by_username("test/user_table").unwrap();
        assert!(get_by_id(userx.id).is_some());
        assert!(userx.picture, "value change by set_picture");
        assert!(is_password(&userx, "5"));
        assert!(!is_password(&userx, "4"));
        assert_eq!(userx.confirm_email, false);
        assert!(set_confirm_email(userx.username.as_str()));
        let userx = get_by_username("test/user_table").unwrap();
        assert!(userx.confirm_email);
        assert_eq!(
            delete_user("test/user_table".to_string()),
            true,
            "must be true, test/user_table user is create"
        );
        assert!(get_by_username("test/user_table").is_none());
        assert_eq!(set_password(userx.username.as_str(), "5"), false);
    }
}
