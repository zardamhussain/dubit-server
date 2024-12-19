use rocket::serde::json::Json;
use rocket::{get, State};

use crate::modals::api_modal::JSONResponse;
use crate::modals::user_modal::{UpdateUser, User, UserWithCredits};
use crate::repos::user_repo::UserRepo;
use crate::utils::auth::Authorization;

#[get("/<user_id>")]
pub async fn get_user_by_id(
    user_id: &str,
    user_repo: &State<UserRepo>,
    auth: Authorization,
) -> Result<Json<JSONResponse<UserWithCredits>>, rocket::http::Status> {
    let user = auth.user;

    if user_id != user.id {
        return Err(rocket::http::Status::Forbidden);
    }

    match user_repo.get_user_with_credits(user_id).await {
        Ok(user) => Ok(Json(JSONResponse::new(
            "User fetched successfully",
            Some(user),
        ))),
        Err(_) => Err(rocket::http::Status::NotFound),
    }
}

#[patch("/<user_id>", data = "<user_data>")]
pub async fn update_user(
    user_id: &str,
    user_data: Json<UpdateUser>,
    user_repo: &State<UserRepo>,
    auth: Authorization,
) -> Result<Json<JSONResponse<User>>, rocket::http::Status> {
    let auth_user = auth.user;

    if user_id != auth_user.id {
        return Err(rocket::http::Status::Forbidden);
    }

    let update_data = user_data.into_inner();

    match user_repo.update_user(user_id, update_data).await {
        Ok(_) => Ok(Json(JSONResponse::new("User updated successfully", None))),
        Err(error) => {
            println!("Error updating user: {:?}", error);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

#[delete("/<user_id>")]
pub async fn delete_user(
    user_id: &str,
    user_repo: &State<UserRepo>,
    auth: Authorization,
) -> Result<Json<JSONResponse<User>>, rocket::http::Status> {
    let auth_user = auth.user;

    if user_id != auth_user.id {
        return Err(rocket::http::Status::Forbidden);
    }

    match user_repo.delete_user(user_id).await {
        Ok(_) => Ok(Json(JSONResponse::new("User deleted successfully", None))),
        Err(error) => {
            println!("Error deleting user: {:?}", error);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}
