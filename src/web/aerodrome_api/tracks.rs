use crate::model::track::Track;
use crate::store::track::{TrackSqliteRepo, TrackRepository};
use rocket::response::status::NotFound;
use rocket::{get, State};
use rocket_contrib::json::Json;

#[get("/<id>")]
pub async fn get_track_by_id(id: &str, tracks: State<'_, TrackSqliteRepo>) -> Result<Json<Track>, NotFound<String>> {
    tracks
        .find_by_id(id)
        .await
        .map(|a| Json(a))
        .map_err(|_| NotFound(format!("No track with id {} found!", id)))
}
