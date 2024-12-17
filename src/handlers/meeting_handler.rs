use rocket::{get, State};
use rocket::serde::json::Json;

use crate::repos::meeting_repo::MeetingRepo;
use crate::modals::meeting_model::Meeting;

#[get("/<room_id>")]
pub async fn get_meeting(
    room_id: &str, 
    meeting_repo: &State<MeetingRepo>
) -> Option<Json<Meeting>> {
    
    println!("room_id: {}", room_id);
    match meeting_repo.get_meeting_by_room_id(room_id).await {
        Ok(meeting) => Some(Json(meeting)),
        Err(_) => None,
    }
}
