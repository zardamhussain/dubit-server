pub mod meeting_handler;
pub mod user_handler;

use rocket::Route;

pub fn user_routes() -> Vec<Route> {
    routes![
        user_handler::get_user_by_id,
        user_handler::update_user,
        user_handler::delete_user
    ]
}

pub fn meeting_routes() -> Vec<Route> {
    routes![meeting_handler::get_meeting,]
}
