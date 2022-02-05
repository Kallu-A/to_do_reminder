use crate::db::pref_table::{Mode, PrefEntity};
use crate::db::todo_table::TodoEntity;

/// Return the vector of to-do but reduce by the size of the pref display fielc
fn limit_display(todos: Vec<TodoEntity>, size: usize) -> Vec<TodoEntity> {
    todos.into_iter().take(size).collect()
}

/// Return the vectors of to-do transform with the different pref possible
/// Don't need a function to sort in Mode::Creation because Diesel already sort by id
pub fn handle_change_list_todo(
    mut todos: Vec<TodoEntity>,
    pref: &PrefEntity,
    limit: bool,
) -> Vec<TodoEntity> {

    todos = match Mode::from_i32(pref.sort).unwrap() {
        Mode::Creation => todos,
        Mode::DatePriority => sort_date_priority(todos),
        Mode::DateProgress => sort_date_progress(todos),
        Mode::DoneNotDone => sort_done_not_done(todos),
        Mode::DatePriorityDone => sort_date_priority_progress(todos)
    };

    if limit {
        todos = limit_display(todos, pref.display as usize);
    }

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
fn sort_done_not_done(todos: Vec<TodoEntity>) -> Vec<TodoEntity> {
    //TODO
    todos
}

/// Sort the vec by following the Mode::DatePriorityDone rule
fn sort_date_priority_progress(todos: Vec<TodoEntity>) -> Vec<TodoEntity> {
    //TODO
    todos
}

#[cfg!(test)]
mod test {
    use crate::db::todo_table::TodoEntity;
    use crate::schema::todo::priority;
    use crate::utils::pref::{sort_date_priority, sort_date_priority_progress, sort_date_progress, sort_done_not_done};

    /// Create a TodoEntity for test purpose
    fn create_todo(id: i32, progress: i32, date: &str, priority: i32) -> TodoEntity {
        TodoEntity {
            id,
            progress,
            id_owner: 0,
            title: " ".to_string(),
            date: date.to_string(),
            priority,
            content: " ".to_string()
        }
    }

    /// Create the vector of TodoEntity for test purpose with value in disorder
    fn create_vec() -> Vec<TodoEntity> {
        let mut vec = Vec::new();
        vec.push(create_todo(0, 10, "10/10/1010", 9));
        vec.push(create_todo(1, 2, "08/10/1010", 2));
        vec.push(create_todo(2, 20, "08/11/1010", 5));
        vec.push(create_todo(3, 10, "08/11/1010", 5));
        vec.push(create_todo(4, 100, "10/10/1008", 3));
        vec.push(create_todo(5, 8, "02/02/1009", 8));
        vec.push(create_todo(6, 20, "02/02/1009", 9));
        vec
    }

    #[test]
    pub fn date_priority() {
        let todos = sort_date_priority(create_vec());
        assert_eq!(todos[0].id, 4);
        assert_eq!(todos[1].id, 5);
        assert_eq!(todos[2].id, 6);
        assert_eq!(todos[3].id, 1);
        assert_eq!(todos[4].id, 0);
        assert_eq!(todos[5].id, 2);
        assert_eq!(todos[6].id, 3);
    }

    #[test]
    pub fn date_progress() {
        let todos = sort_date_progress(create_vec());
        assert_eq!(todos[0].id, 4);
        assert_eq!(todos[1].id, 5);
        assert_eq!(todos[2].id, 6);
        assert_eq!(todos[3].id, 1);
        assert_eq!(todos[4].id, 0);
        assert_eq!(todos[5].id, 3);
        assert_eq!(todos[6].id, 2);
    }

    #[test]
    pub fn done_not_done() {
        let todos = sort_done_not_done(create_vec());
        assert_eq!(todos[0].id, 1);
        assert_eq!(todos[1].id, 5);
        assert_eq!(todos[2].id, 0);
        assert_eq!(todos[3].id, 3);
        assert_eq!(todos[4].id, 2);
        assert_eq!(todos[5].id, 6);
        assert_eq!(todos[6].id, 4);
    }

    #[test]
    pub fn date_priority_progress() {
        let todos = sort_date_priority_progress(create_vec());
        assert_eq!(todos[0].id, 4);
        assert_eq!(todos[1].id, 5);
        assert_eq!(todos[2].id, 6);
        assert_eq!(todos[3].id, 1);
        assert_eq!(todos[4].id, 0);
        assert_eq!(todos[5].id, 3);
        assert_eq!(todos[6].id, 2);
    }
}
