use crate::db::pref_table::{Mode, PrefEntity};
use crate::db::todo_table::TodoEntity;

/// Return the vector of to-do but reduce by the size of the pref display fielc
fn limit_display(todos: Vec<TodoEntity>, size: usize) -> Vec<TodoEntity> {
    todos.into_iter().take(size).collect()
}

/// Return the vectors of to-do transform with the different pref possible
pub fn handle_change_list_todo(
    mut todos: Vec<TodoEntity>,
    pref: &PrefEntity,
    limit: bool,
) -> Vec<TodoEntity> {

    todos = match Mode::from_i32(pref.sort).unwrap() {
        Mode::Creation => sort_creation(todos),
        Mode::DatePriority => sort_date_priority(todos),
        Mode::DateProgress => sort_date_progress(todos),
        Mode::DoneNotDone => sort_not_done(todos),
        Mode::DatePriorityDone => sort_date_priority_done(todos)
    };

    if limit {
        todos = limit_display(todos, pref.display as usize);
    }

    todos
}

/// Sort the vec by following the Mode::Creation rule
fn sort_creation(todos: Vec<TodoEntity>) -> Vec<TodoEntity> {
    //TODO
    todos
}

/// Sort the vec by following the Mode::DatePriority rule
fn sort_date_priority(todos: Vec<TodoEntity>) -> Vec<TodoEntity> {
    //TODO
    todos
}

/// Sort the vec by following the Mode::Progress rule
fn sort_date_progress(todos: Vec<TodoEntity>) -> Vec<TodoEntity> {
    //TODO
    todos
}

/// Sort the vec by following the Mode::DoneNotDone rule
fn sort_not_done(todos: Vec<TodoEntity>) -> Vec<TodoEntity> {
    //TODO
    todos
}

/// Sort the vec by following the Mode::DatePriorityDone rule
fn sort_date_priority_done(todos: Vec<TodoEntity>) -> Vec<TodoEntity> {
    //TODO
    todos
}
