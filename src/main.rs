#[macro_use]
extern crate rocket;

mod handlers;
mod modals;
mod repos;
mod utils;

use rocket::launch;

use dotenv::dotenv;
use std::sync::Arc;

use repos::meeting_repo::MeetingRepo;
use repos::user_repo::UserRepo;
use utils::{conn::create_pool, gzip::Gzip};

use handlers::{meeting_routes, user_routes};

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let pool = create_pool().await;
    let arc_pool: Arc<sqlx::Pool<sqlx::Postgres>> = Arc::new(pool);

    let user_repo = UserRepo::new(Arc::clone(&arc_pool), None);
    let meeting_repo = MeetingRepo::new(Arc::clone(&arc_pool), None);

    rocket::build()
        .attach(Gzip)
        .manage(user_repo)
        .manage(meeting_repo)
        .mount("/users", user_routes())
        .mount("/meetings", meeting_routes())
}
