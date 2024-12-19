use chrono::Utc;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use std::env;

use crate::modals::user_modal::{CreateUser, Gender, User};
use crate::repos::user_repo::UserRepo;
use crate::utils::firebase::verify_firebase_id_token;

pub struct Authorization {
    pub user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authorization {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        let header = req.headers().get_one("Authorization");

        let user_repo = req.rocket().state::<UserRepo>();
        if user_repo.is_none() {
            return Outcome::Error((Status::InternalServerError, ()));
        }
        let user_repo = user_repo.unwrap();

        if let Some(header_value) = header {
            if header_value.starts_with("Bearer ") {
                let token = header_value[7..].to_string();

                if let Some(user) = is_valid_token(&token, user_repo).await {
                    return Outcome::Success(Authorization { user });
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}

async fn is_valid_token(token: &str, user_repo: &UserRepo) -> Option<User> {
    let firebase_project_id = env::var("FIREBASE_PROJECT_ID")
        .expect("FIREBASE_PROJECT_ID environment variable must be set");

    match verify_firebase_id_token(token, &firebase_project_id).await {
        Some(firebase_user) => match user_repo.get_user_by_id(&firebase_user.user_id).await {
            Ok(user) => Some(user),
            Err(_) => {
                let display_name = firebase_user.name.unwrap_or("".to_string());
                let email = firebase_user.email.unwrap();
                let photo_url = firebase_user.picture;

                let new_user = CreateUser {
                    email,
                    display_name: Some(display_name),
                    photo_url,
                    dob: None,
                    is_whitelisted: false,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    gender: Gender::Male,
                };

                match user_repo.create_user(new_user).await {
                    Ok(user) => Some(user),
                    Err(error) => {
                        println!("Error verifying token: {}", error);
                        None
                    }
                }
            }
        },
        None => {
            println!("Error verifying token:");
            None
        }
    }
}
