use rocket::{get, State};
use rocket::serde::json::Json;

use crate::repos::user_repo::UserRepo;
use crate::modals::user_modal::User;
use crate::utils::auth::Authorization;

#[get("/<email>")]
pub async fn get_user(
    email: &str, 
    user_repo: &State<UserRepo>,
    auth: Authorization
) -> Option<Json<User>> {
    println!("auth: {}", auth.token);
    println!("email: {}", email);
    match user_repo.get_user_by_email(email).await {
        Ok(user) => Some(Json(user)),
        Err(_) => None,
    }

}