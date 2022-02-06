use chrono::NaiveDate;
use crate::db::todo_table::TodoEntity;
use rocket::serde::Serialize;
use time::OffsetDateTime;

/// Allow not only to select what information is send to the user
/// but it also calculat the state of the to-do with the current date
pub fn calculate_date_state(todos: Vec<TodoEntity>) -> Vec<TodoDisplay>{
    let mut res: Vec<TodoDisplay> = Vec::new();
    let current_date = NaiveDate::parse_from_str(
        OffsetDateTime::now_utc().date().to_string().as_str(),
        "%Y-%m-%d").unwrap();
    for todo in todos  {
        let mut state = 0;
        if NaiveDate::parse_from_str(todo.date.as_str(), "%d/%m/%Y").unwrap() < current_date {
            //date is passed
            state = if todo.progress == 100 { 2 } else { 1 };
        } else {
            state = if todo.progress == 100 { 3 } else { state };
        }

        res.push(
            TodoDisplay {
                id: todo.id,
                progress: todo.progress,
                title: todo.title.to_owned(),
                date: todo.date.to_owned(),
                priority: todo.priority,
                content: todo.content.to_owned(),
                state,
            }
        )
    }
    res
}

/// State will represent the state of the to-do from the current date
/// 0 = noting
/// 1 = date passed and to-do not done
/// 2 = date passed and to-do done
/// 3 = to-do done
#[derive(Serialize)]
pub struct TodoDisplay {
    pub id: i32,
    pub progress: i32,
    pub title: String,
    pub date: String,
    pub priority: i32,
    pub content: String,
    pub state: i32
}