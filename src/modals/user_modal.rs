use sqlx::{Type, FromRow};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


#[derive(Debug, Type, Serialize, Deserialize)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
enum Gender {
    Male,
    Female,
    Undisclosed,
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub display_name: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    photo_url: Option<String>,
    dob: Option<DateTime<Utc>>,
    gender: Option<Gender>,
    is_whitelisted: Option<bool>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    is_deleted: Option<bool>,
    last_sign_in_at: Option<DateTime<Utc>>,
}
