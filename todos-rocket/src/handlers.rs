use std::arch::asm;
use std::io::Cursor;

use chrono::Utc;
use rocket::{get, post, Response, State};
use rocket::http::Status;
use rocket::response::status;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::data::{AppState, Todo};

#[post("/todos", data = "<todo>")]
pub async fn create(mut todo: Json<Todo>, app_state: &State<AppState>) -> Result<Json<Todo>, String> {
    let id = Uuid::new_v4().to_string();
    todo.id = Some(id.clone());
    todo.create_time = Some(Utc::now());
    todo.completed = Some(false);

    if let Ok(mut todos) = app_state.todos.lock() {
        todos.push(todo.clone().into_inner());
    };

    Ok(Json(todo.into_inner()))
}

#[get("/todos/<id>")]
pub async fn get_by_id(id: String, app_state: &State<AppState>) -> Result<Json<Todo>, NotFound<()>> {
    app_state.todos.lock()
        .map(|todos| todos.clone().into_iter().find(|todo| todo.id == Some(id.clone())))
        .map(|todo| if todo.is_none() { Err(NotFound(())) } else { Ok(Json(todo.unwrap())) })
        .map_err(|_| Status::InternalServerError )
        .unwrap()
}

#[get("/todos?<completed>")]
pub async fn get_all(completed: bool, app_state: &State<AppState>) -> Result<Json<Vec<Todo>>, NotFound<()>> {
    if let Ok(todos) = app_state.todos.lock() {
        let active_todos: Vec<Todo> = todos.clone().into_iter().filter(|todo| todo.completed == Some(completed)).collect();
        Ok(Json(active_todos))
    } else {
        Err(NotFound(()))
    }
}
