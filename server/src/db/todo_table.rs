use rocket::serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::db::handler;
use crate::schema::todo;
use crate::schema::todo::dsl::*;


/// Struct who represent a to-do
#[derive(Queryable, Serialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "todo"]
pub struct TodoEntity {
    pub id: i32,
    pub owner: String,
    pub title: String,
    pub date: String,
    pub content: String,
    pub priority: i32,
}

/// Return the to-do of the id, none if he doesn't exist
pub fn get_by_id(id_find: i32) -> Option<TodoEntity> {
    let con = &mut handler::establish_connection();
    match todo.filter(id.eq(id_find)).load::<TodoEntity>(con) {
        Ok(mut res) => {
            if !res.is_empty() {
                res.drain(0..1).next()
            } else {
                None
            }
        }
        Err(_) => None
    }
}

/// Return all the to-do of the owner, none if he doesn't exist
pub fn get_by_owner(owner_find: &str) -> Vec<TodoEntity> {
    let con = &mut handler::establish_connection();
    todo.filter(owner.eq(owner_find)).load::<TodoEntity>(con)
        .expect("error loading the user todo")
}

/// Try to delete a to-do with is id
pub fn delete_by_id(id_delete: i32) -> bool {
    let con = &mut handler::establish_connection();
    let num_deleted = diesel::delete(todo.filter(id.eq(id_delete)))
        .execute(con)
        .expect("Error deleting todo");

    num_deleted > 0
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
    pub fn check() {
    }
}