#[macro_use] extern crate rocket;

mod repos;
mod modals;
mod handlers;
mod utils;

use rocket::launch;

use dotenv::dotenv;
use std::sync::Arc;

use utils::{conn::create_pool, fairing::Logger};
use repos::user_repo::UserRepo;
use repos::meeting_repo::MeetingRepo;

use handlers::{user_routes, meeting_routes};

#[launch]
async fn rocket() -> _ {

    dotenv().ok();
    
    let pool = create_pool().await;
    let arc_pool: Arc<sqlx::Pool<sqlx::Postgres>> = Arc::new(pool);

    let user_repo = UserRepo::new(Arc::clone(&arc_pool), None);
    let meeting_repo = MeetingRepo::new(Arc::clone(&arc_pool), None);
    
    rocket::build()
        .attach(Logger)
        .manage(user_repo)
        .manage(meeting_repo)
        .mount("/users", user_routes())
        .mount("/meetings", meeting_routes())
        
}