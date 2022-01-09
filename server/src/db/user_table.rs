use diesel::prelude::*;
use regex::Regex;
use rocket::serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::db::handler;
use crate::schema::user;
use crate::schema::user::dsl::*;

pub const DEFAULT_PATH: &str = "../static/image/profil/default.png";

/// Struct get by all the getter of the database with 4 fields of the table
#[derive(Queryable, Serialize)]
pub struct UserEntity {
    pub username: String,
    pub password: String,
    pub perm: bool,
    pub picture: bool,
}

impl UserEntity {
    /// Return path of the file of the profile picture if has it or else default.png
    pub fn get_path(&self) -> String {
        let mut path = "../static/image/profil/".to_string();
        // set picture to the user if is has one else default
        if self.picture {
            path.push_str(self.username.as_str());
        } else {
            path.push_str("default.png");
        }
        path
    }
}
/// Struct to put a new user on the table User
#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUserEntity<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub perm: bool,
    pub picture: bool,
}

/// Struct used to create a new User by form with 2 password to make user confirm is password
#[derive(Debug, FromForm, Serialize)]
pub struct UsersForm<'a> {
    pub(crate) username_f: &'a str,
    pub(crate) password_f: Password<'a>,
}

/// Struct to let the user try to login
#[derive(FromForm, Deserialize)]
pub struct UsersLogin<'a> {
    pub(crate) username_l: &'a str,
    pub(crate) password_l: &'a str,
}

/// Needed at the creation to ensure their is no typo in the password
#[derive(Debug, FromForm, Serialize)]
pub struct Password<'v> {
    pub(crate) first: &'v str,
    pub(crate) second: &'v str,
}

/// Return the username with the name in param or None if he doesn't exist
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

/// Return all the User on the Table `user`
pub fn get_all() -> Vec<UserEntity> {
    let connection = &mut handler::establish_connection();
    user.load::<UserEntity>(connection)
        .expect("Error loading user")
}

/// Try to delete the username in args and return true if the delete was successful else false
pub fn delete(username_delete: String) -> bool {
    let connection = &mut handler::establish_connection();
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
/// password
/// admin for the permission
pub fn create_user_perm(username_x: &str, password_x: &str, admin_x: bool) -> usize {
    let conn = &handler::establish_connection();

    let regex = Regex::new("^test*").unwrap();
    if username_x == "default.png" || regex.is_match(username_x) && !admin_x {
        return 0;
    }

    let new_user = NewUserEntity {
        username: username_x,
        password: password_x,
        perm: admin_x,
        picture: false,
    };

    diesel::insert_into(user::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user")
}

/// Allow to also create a user,
/// but without the need of specifying a boolean
/// value for perm default value will be false
pub fn create_user(username_a: &str, password_a: &str) -> usize {
    create_user_perm(username_a, password_a, false)
}

/// Allow to change the value of a user picture book to change if he as a profile picture or not
pub fn set_picture(user_x: &UserEntity, pic: bool) -> bool {
    let conn = &mut handler::establish_connection();
    let new = NewUserEntity {
        username: user_x.username.as_str(),
        password: user_x.password.as_str(),
        perm: user_x.perm,
        picture: pic,
    };
    diesel::replace_into(user).values(new).execute(conn).is_ok()
}

#[cfg(test)]
mod tests {
    use crate::db::user_table::{delete, set_picture};
    use crate::{create_user_perm, get};
    use std::panic;

    #[test]
    fn check() {
        panic::set_hook(Box::new(|err| {
            delete("test/user_table".to_string());
            println!("\n{}", err.to_string());
            println!("{}", err.location().unwrap().to_string());
        }));

        assert!(get("test/user_table").is_none(), "reserved username");
        assert_eq!(create_user_perm("test/user_table", "1", true), 1);
        let userx = get("test/user_table");
        assert!(userx.is_some(), "just create");
        let userx = userx.unwrap();
        assert_eq!(userx.picture, false, "default value");
        set_picture(&userx, true);
        let userx = get("test/user_table").unwrap();
        assert!(userx.picture, "value change by set_picture");
        assert_eq!(
            delete("test/user_table".to_string()),
            true,
            "must be true, test/user_table user is create"
        );
        assert!(get("test/user_table").is_none());
    }
}