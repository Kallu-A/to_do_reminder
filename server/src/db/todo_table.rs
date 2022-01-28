use rocket::serde::Serialize;
use crate::db::handler;
use diesel::prelude::*;
use crate::schema::todo;
use crate::schema::todo::dsl::*;

/// Struct who represent a to-do
#[derive(Queryable, Serialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "todo"]
pub struct TodoEntity {
    pub id: i32,
    pub progress: i32,
    pub owner: String,
    pub title: String,
    pub date: String,
    pub priority: i32,
    pub content: String,
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

/// Try to delete all the to-do a a user
pub fn delete_by_owner(owner_delete: &str) -> usize {
    let con = &mut handler::establish_connection();
    let num_deleted = diesel::delete(todo.filter(owner.eq(owner_delete)))
        .execute(con)
        .expect("Error deleting todo");

    num_deleted
}


#[cfg(test)]
mod tests {

    #[test]
    pub fn check() {
    }
}