use rocket::time::Date;
use sqlx::{Type, FromRow};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Type, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Undisclosed,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo_url: Option<String>,
    pub dob: Option<DateTime<Utc>>,
    pub gender: Option<Gender>,
    pub is_whitelisted: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_deleted: Option<bool>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    // pub user_credits: Option<serde_json::Value>
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UserCredit {
    pub credits: Option<i64>,
    pub seconds_remaining: f64
}


#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UserWithCredits {
    pub id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo_url: Option<String>,
    pub dob: Option<DateTime<Utc>>,
    pub gender: Option<Gender>,
    pub is_whitelisted: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_deleted: Option<bool>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub user_credits: Option<serde_json::Value>
}


#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct CreateUser {
    pub email: String,
    pub display_name: Option<String>,
    pub photo_url: Option<String>,
    pub dob: Option<DateTime<Utc>>,
    pub is_whitelisted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub gender: Gender
}