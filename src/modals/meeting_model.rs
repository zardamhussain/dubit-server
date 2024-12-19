use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Meeting {
    pub id: String,
    pub room_id: String,
    pub participant_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub end_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
    pub is_deleted: Option<bool>,
    pub meeting_details: Option<serde_json::Value>,
    pub meeting_agenda: Option<String>,
    pub recorded_url: Option<String>,
}
