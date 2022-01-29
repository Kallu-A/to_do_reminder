use crate::db::handler;
use crate::schema::todo;
use crate::schema::todo::dsl::*;
use diesel::prelude::*;
use rocket::serde::Serialize;
use std::cmp::{max, min};

/// Struct who represent a to-do
#[derive(Queryable, Serialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "todo"]
pub struct TodoEntity {
    pub id: i32,
    pub progress: i32,
    pub id_owner: i32,
    pub title: String,
    pub date: String,
    pub priority: i32,
    pub content: String,
}

/// Stuct to insert a to-do
#[derive(Insertable)]
#[table_name = "todo"]
struct NewTodoEntity {
    pub progress: i32,
    pub id_owner: i32,
    pub title: String,
    pub date: String,
    pub priority: i32,
    pub content: String,
}

/// Form for when you want to create a to-do
#[derive(FromForm, Serialize)]
pub struct CreateTodo<'a> {
    pub title_x: &'a str,
    pub content_x: &'a str,
    pub date_x: &'a str,
    pub priority_x: i32,
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
        Err(_) => None,
    }
}

/// Return all the to-do of the owner, none if he doesn't exist
pub fn get_by_owner(owner_find: i32) -> Vec<TodoEntity> {
    let con = &mut handler::establish_connection();
    todo.filter(id_owner.eq(owner_find))
        .load::<TodoEntity>(con)
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
pub fn delete_by_owner(owner_delete: i32) -> usize {
    let con = &mut handler::establish_connection();
    diesel::delete(todo.filter(id_owner.eq(owner_delete)))
        .execute(con)
        .expect("Error deleting todo")
}

/// Try to delete all the to-do a a user
pub fn delete_done_by_owner(owner_delete: i32) -> usize {
    let con = &mut handler::establish_connection();
    diesel::delete(
        todo.filter(id_owner.eq(owner_delete))
            .filter(progress.eq(100)),
    )
    .execute(con)
    .expect("Error deleting todo done")
}

/// Create the to-do
pub fn create_todo(
    id_owner_x: i32,
    title_x: &str,
    date_x: &str,
    priority_x: i32,
    content_x: &str,
) -> usize {
    let con = &mut handler::establish_connection();

    let new_todo = NewTodoEntity {
        progress: 0,
        id_owner: id_owner_x,
        title: title_x.to_string(),
        date: date_x.to_string(),
        priority: priority_x,
        content: content_x.to_string(),
    };

    diesel::insert_into(todo::table)
        .values(&new_todo)
        .execute(con)
        .expect("Error saving new todo")
}

/// set the progress to the value normalise in [0; 100]
pub fn set_progress(todo_x: &mut TodoEntity, progress_x: i32) -> bool {
    let progress_x = max(min(100, progress_x), 0);
    todo_x.progress = progress_x;

    let con = &mut handler::establish_connection();
    todo_x.save_changes::<TodoEntity>(con).is_ok()
}

#[cfg(test)]
mod tests {
    use crate::db::todo_table::{
        create_todo, delete_by_id, delete_by_owner, delete_done_by_owner, get_by_id, get_by_owner,
        set_progress,
    };
    use std::panic;

    #[test]
    pub fn check() {
        panic::set_hook(Box::new(|err| {
            delete_by_owner(-1);
            println!("\n{}", err.to_string());
            println!("{}", err.location().unwrap().to_string());
        }));

        assert!(get_by_id(-1).is_none());
        assert!(get_by_owner(-1).is_empty());
        assert_eq!(delete_by_id(-1), false);
        assert_eq!(delete_by_owner(-1), 0);

        assert_eq!(create_todo(-1, "test", "01/01/1001", 1, "welcome"), 1);
        assert_eq!(create_todo(-1, "test2", "02/02/1002", 2, "welcome2"), 1);
        assert_eq!(create_todo(-1, "test3", "03/03/3003", 3, "welcome3"), 1);
        assert_eq!(create_todo(-1, "test4", "04/04/4004", 4, "welcome4"), 1);

        let mut todos = get_by_owner(-1);
        assert_eq!(todos.len(), 4);

        todos.iter().for_each(|t| {
            assert!(get_by_id(t.id).is_some());
        });

        assert_eq!(todos[0].progress, 0);
        assert!(set_progress(&mut todos[0], 100));

        let todo = get_by_id(todos[0].id).unwrap();
        assert_eq!(todo.progress, 100);

        assert_eq!(delete_done_by_owner(-1), 1);
        assert_eq!(delete_by_id(todos[0].id), false);
        assert!(delete_by_id(todos[1].id));
        assert_eq!(get_by_owner(-1).len(), 2);
        assert_eq!(delete_by_owner(-1), 2);
    }
}
