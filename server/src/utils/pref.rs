use crate::db::pref_table::{Mode, PrefEntity};
use crate::db::todo_table::TodoEntity;
use chrono::NaiveDate;

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
        Mode::DatePriorityDone => sort_date_priority_progress(todos),
    };

    if limit {
        todos = limit_display(todos, pref.display as usize);
    }

    todos
}

/// Sort by date the vector
fn sort_algorithm_date(mut todos: Vec<TodoEntity>) -> Vec<TodoEntity> {
    let mut len = todos.len();

    while 1 < len {
        for i in 1..len {
            if NaiveDate::parse_from_str(todos[i - 1].date.as_str(), "%d/%m/%Y").unwrap()
                > NaiveDate::parse_from_str(todos[i].date.as_str(), "%d/%m/%Y").unwrap()
            {
                todos.swap(i - 1, i);
            }
        }
        len -= 1;
    }

    todos
}

/// Sort the vec by following the Mode::DatePriority rule
fn sort_date_priority(mut todos: Vec<TodoEntity>) -> Vec<TodoEntity> {
    todos = sort_algorithm_date(todos);

    let mut len = todos.len();
    while 1 < len {
        for i in 1..len {
            if todos[i - 1].date == todos[i].date && todos[i - 1].priority < todos[i].priority {
                todos.swap(i - 1, i);
            }
        }
        len -= 1;
    }

    todos
}

/// Sort the vec by following the Mode::Progress rule
fn sort_date_progress(mut todos: Vec<TodoEntity>) -> Vec<TodoEntity> {
    todos = sort_algorithm_date(todos);
    let mut len = todos.len();
    while 1 < len {
        for i in 1..len {
            if todos[i - 1].date == todos[i].date && todos[i - 1].progress > todos[i].progress {
                todos.swap(i - 1, i);
            }
        }
        len -= 1;
    }

    todos
}

/// Sort the vec by following the Mode::DoneNotDone rule
fn sort_done_not_done(mut todos: Vec<TodoEntity>) -> Vec<TodoEntity> {
    let mut len = todos.len();

    while 1 < len {
        for i in 1..len {
            if todos[i - 1].progress > todos[i].progress {
                todos.swap(i - 1, i);
            }
        }
        len -= 1;
    }

    todos
}

/// Sort the vec by following the Mode::DatePriorityDone rule
fn sort_date_priority_progress(mut todos: Vec<TodoEntity>) -> Vec<TodoEntity> {
    todos = sort_date_priority(todos);
    let mut len = todos.len();
    while 1 < len {
        for i in 1..len {
            if todos[i - 1].date == todos[i].date
                && todos[i - 1].priority == todos[i].priority
                && todos[i - 1].progress > todos[i].progress
            {
                todos.swap(i - 1, i);
            }
        }
        len -= 1;
    }
    todos
}

#[cfg(test)]
mod test {
    use crate::db::todo_table::TodoEntity;
    use crate::utils::pref::{
        sort_date_priority, sort_date_priority_progress, sort_date_progress, sort_done_not_done,
    };

    /// Create a TodoEntity for test purpose
    fn create_todo(id: i32, progress: i32, date: &str, priority: i32) -> TodoEntity {
        TodoEntity {
            id,
            progress,
            id_owner: 0,
            title: " ".to_string(),
            date: date.to_string(),
            priority,
            content: " ".to_string(),
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
        assert_eq!(todos[1].id, 6);
        assert_eq!(todos[2].id, 5);
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
    pub fn date_and_priority_progress() {
        let todos = sort_date_priority_progress(create_vec());
        assert_eq!(todos[0].id, 4);
        assert_eq!(todos[1].id, 6);
        assert_eq!(todos[2].id, 5);
        assert_eq!(todos[3].id, 1);
        assert_eq!(todos[4].id, 0);
        assert_eq!(todos[5].id, 3);
        assert_eq!(todos[6].id, 2);
    }
}
