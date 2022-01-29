use crate::db::handler;
use crate::db::pref_table::Mode::{Creation, Date, DatePriority, Priority};
use crate::schema::pref;
use crate::schema::pref::dsl::*;
use diesel::prelude::*;
use rocket::serde::Serialize;

/// Struct who represent the pref
#[derive(Queryable, Serialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "pref"]
pub struct PrefEntity {
    pub id: i32,
    pub id_user: i32,
    pub sort: i32,
    pub display: i32,
}

/// Struct to create a new pref
#[derive(Insertable)]
#[table_name = "pref"]
pub struct NewPrefEntity {
    pub id_user: i32,
    pub sort: i32,
    pub display: i32,
}

/// Return the pref of the id if one exist
pub fn get_pref_from_owner(id_user_x: i32) -> Option<PrefEntity> {
    let con = &mut handler::establish_connection();
    match pref.filter(id_user.eq(id_user_x)).load::<PrefEntity>(con) {
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

/// Create the pref of a user with the default settings
pub fn create_pref(id_user_x: i32) -> usize {
    let con = &mut handler::establish_connection();

    let new_pref = NewPrefEntity {
        id_user: id_user_x,
        sort: DEFAULT_MODE,
        display: DEFAULT_MODE,
    };

    diesel::insert_into(pref::table)
        .values(&new_pref)
        .execute(con)
        .expect("Error saving pref")
}

/// Try to the delete the pref of a user
pub fn delete_pref(id_userx: i32) -> usize {
    let con = &mut handler::establish_connection();
    diesel::delete(pref.filter(id_user.eq(id_userx)))
        .execute(con)
        .expect("Error deleting pref")
}

pub const DEFAULT_MODE: i32 = 0;

/// Contains all the different mode of sort & display
pub enum Mode {
    Creation,
    Date,
    Priority,
    DatePriority,
}

impl Mode {
    /// Convert the i32 to the mode
    pub fn from_i32(value: i32) -> Result<Mode, i32> {
        match value {
            0 => Ok(Creation),
            1 => Ok(Date),
            2 => Ok(Priority),
            3 => Ok(DatePriority),
            x => Err(x),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::db::pref_table::{create_pref, delete_pref, get_pref_from_owner, DEFAULT_MODE};
    use crate::db::user_table::delete_user;
    use crate::{create_user_perm, get_by_username};
    use std::panic;

    #[test]
    pub fn check() {
        panic::set_hook(Box::new(|err| {
            delete_pref(-1);
            println!("\n{}", err.to_string());
            println!("{}", err.location().unwrap().to_string());
        }));

        assert!(get_pref_from_owner(-1).is_none());
        assert_eq!(create_pref(-1), 1);
        let pref = get_pref_from_owner(-1);
        assert!(pref.is_some());
        let pref = pref.unwrap();
        assert_eq!(pref.id_user, -1);
        assert_eq!(pref.display, DEFAULT_MODE);
        assert_eq!(pref.sort, DEFAULT_MODE);
        assert_eq!(delete_pref(-1), 1);
        assert!(get_pref_from_owner(-1).is_none());
    }

    #[test]
    pub fn link_user() {
        let usernamex = "test/pref_table";
        panic::set_hook(Box::new(|err| {
            delete_user(usernamex.to_string());
            println!("\n{}", err.to_string());
            println!("{}", err.location().unwrap().to_string());
        }));

        assert!(get_by_username(usernamex).is_none(), "reserved username");
        assert_eq!(create_user_perm(usernamex, "1", "yo@gmail.com", true), 1);
        let userx = get_by_username(usernamex);
        assert!(userx.is_some(), "just create");
        let userx = userx.unwrap();
        assert!(get_pref_from_owner(userx.id).is_some());
        assert_eq!(
            delete_user(usernamex.to_string()),
            true,
            "must be true, test/pref_table user is create"
        );
        assert!(get_pref_from_owner(userx.id).is_none());
    }
}
