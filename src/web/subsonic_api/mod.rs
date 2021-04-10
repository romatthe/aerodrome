use rocket::State;
use crate::store::track::{TrackSqliteRepo, TrackRepository};
use rocket::response::status::NotFound;

#[get("/getSong/<id>")]
pub async fn get_song(id: &str, tracks: State<'_, TrackSqliteRepo>) -> Result<Json<Track>, NotFound<String>> {
    tracks
        .find_by_id(id)
        .await
        .map(|a| Json(a))
        .map_err(|_| NotFound(format!("No track with id {} found!", id)))
}