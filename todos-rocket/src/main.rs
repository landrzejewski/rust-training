use rocket::{Build, launch, Rocket, routes};
use crate::data::AppState;
use handlers::{create, get_by_id, get_all};

mod data;
mod handlers;

#[launch]
fn rocket() -> Rocket<Build> {
    let app_state = AppState::new();
    rocket::build()
        .manage(app_state)
        .mount("/api", routes![create, get_by_id, get_all])
}
