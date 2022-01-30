use crate::db::handler;
use crate::db::pref_table::Mode::{Creation, Date, DateDonePriorityProgress, DatePriority, DatePriorityDone, DatePriorityProgress, DatePriorityProgressDone, DateProgress, DoneNotDone, Priority};
use crate::schema::pref;
use crate::schema::pref::dsl::*;
use diesel::prelude::*;
use rocket::serde::Serialize;

/// Struct who represent the pref
#[derive(Queryable, Serialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "pref"]
pub struct PrefEntity {
    pub id: i32,
    pub id_owner: i32,
    pub sort: i32,
    pub display: i32,
}

#[derive(Insertable)]
#[table_name = "pref"]
pub struct NewPrefEntity {
    pub id_owner: i32,
    pub sort: i32,
    pub display: i32,
}

/// Return the pref of the id if one exist
pub fn get_pref_from_owner(id_x: i32) -> Option<PrefEntity> {
    let con = &mut handler::establish_connection();
    match pref.filter(id_owner.eq(id_x)).load::<PrefEntity>(con) {
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
pub fn create_pref(id_x: i32) -> usize {
    let con = &mut handler::establish_connection();

    let new_pref = NewPrefEntity {
        id_owner: id_x,
        sort: DEFAULT_MODE,
        display: 3,
    };

    diesel::insert_into(pref::table)
        .values(&new_pref)
        .execute(con)
        .expect("Error saving pref")
}

/// Try to the delete the pref of a user
pub fn delete_pref(id_x: i32) -> usize {
    let con = &mut handler::establish_connection();
    diesel::delete(pref.filter(id_owner.eq(id_x)))
        .execute(con)
        .expect("Error deleting pref")
}

/// Update the value from a PrefEntity
pub fn update_pref(pref_x: &PrefEntity) -> bool {
    let con = &mut handler::establish_connection();
    pref_x.save_changes::<PrefEntity>(con).is_ok()
}

pub const DEFAULT_MODE: i32 = 0;


/// Contains all the different mode of sort & display
/// `Creation` default mode display with the id in the database to the first created to the last
/// `DatePriority` sort by date and if 2 have the same the more important will go before
/// `DateProgress` sort by date and if 2 have the same the less advanced is show before
/// `DoneNotDone` sort by progress first will be the less progress to the done sort by the progress value
/// `DatePriorityProgress` sort by date and if 2 have the same date sort by priority and then progress
pub enum Mode {
    Creation,
    DatePriority,
    DateProgress,
    DoneNotDone,
    DatePriorityDone,

}

impl Mode {
    /// Convert the i32 to the mode
    pub fn from_i32(value: i32) -> Result<Mode, i32> {
        match value {
            0 => Ok(Creation),
            1 => Ok(DatePriority),
            2 => Ok(DateProgress),
            3 => Ok(DoneNotDone),
            4 => Ok(DatePriorityDone),
            x => Err(x),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::db::pref_table::{
        create_pref, delete_pref, get_pref_from_owner, update_pref, DEFAULT_MODE,
    };
    use crate::db::user_table::delete_user;
    use crate::{create_user_perm, get_by_username};
    use std::panic;

    #[test]
    pub fn check() {
        panic::set_hook(Box::new(|err| {
            delete_pref(-2);
            println!("\n{}", err.to_string());
            println!("{}", err.location().unwrap().to_string());
        }));

        assert!(get_pref_from_owner(-2).is_none());
        assert_eq!(create_pref(-2), 1);
        let pref = get_pref_from_owner(-2);
        let mut pref = pref.unwrap();
        assert_eq!(pref.id_owner, -2);
        assert_eq!(pref.display, 3);
        assert_eq!(pref.sort, DEFAULT_MODE);
        pref.display = 2;
        pref.sort = 3;
        assert!(update_pref(&pref));
        let pref = get_pref_from_owner(-2).unwrap();
        assert_eq!(pref.display, 2);
        assert_eq!(pref.sort, 3);
        assert_eq!(delete_pref(-2), 1);
        assert!(get_pref_from_owner(-2).is_none());
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
