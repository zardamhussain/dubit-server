use rocket::{get, State};
use rocket::serde::json::Json;
use serde::Serialize;

use crate::repos::user_repo::UserRepo;
use crate::modals::user_modal::{User, UserWithCredits};
use crate::utils::auth::Authorization;
use crate::modals::api_modal::JSONResponse;


#[get("/<user_id>")]
pub async fn get_user_by_id(
    user_id: &str,
    user_repo: &State<UserRepo>,
    auth: Authorization
) -> Result<Json<JSONResponse<UserWithCredits>>, rocket::http::Status> {
    

    let user = auth.user;

    if user_id != user.id {
        return Err(rocket::http::Status::Forbidden);
    }

    match user_repo.get_user_with_credits(user_id).await {
        Ok(user) => Ok(Json(JSONResponse::new("User fetched successfully", user))),
        Err(_) => Err(rocket::http::Status::NotFound),
    }
}
