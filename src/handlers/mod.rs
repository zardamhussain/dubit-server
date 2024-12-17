pub mod user_handler;
pub mod meeting_handler;

use rocket::Route;

pub fn user_routes() -> Vec<Route> {
    routes![
        user_handler::get_user,
    ]
}

pub fn meeting_routes() -> Vec<Route> {
    routes![
        meeting_handler::get_meeting,
    ]
}
