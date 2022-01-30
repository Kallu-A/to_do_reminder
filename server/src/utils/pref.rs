use crate::db::todo_table::TodoEntity;

/// Return the vector of to-do but reduce by the size of the pref display fielc
pub fn limit_display(todos: Vec<TodoEntity>, size: usize) -> Vec<TodoEntity> {
    todos.into_iter().take(size).collect()
}