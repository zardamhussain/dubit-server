use std::sync::Arc;
use sqlx::{Pool, Postgres, Error};
use crate::modals::meeting_model::Meeting;

#[derive(Debug)]
pub struct MeetingRepo {
    pool: Arc<Pool<Postgres>>,
    table_name: String,
}

impl MeetingRepo {
    pub fn new(
        pool: Arc<Pool<Postgres>>,
         table_name: Option<&str>
    ) -> Self {
    
        Self {
            pool,
            table_name: table_name.unwrap_or("meetings").to_string(),
        }
    
    }

    pub async fn get_meeting_by_room_id(
        &self, 
        room_id: &str
    ) -> Result<Meeting, Error> {
    
        let query = format!("SELECT * FROM {} WHERE room_id = $1", self.table_name);
        let meeting = sqlx::query_as::<_, Meeting>(&query)
            .bind(room_id)
            .fetch_one(self.pool.as_ref())
            .await?;
        Ok(meeting)
    
    }

}

