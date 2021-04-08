use rocket::{get, State};
use crate::store::album::{AlbumRepository, AlbumSqliteRepo};
use crate::model::album::Album;
use rocket::response::status::NotFound;
use rocket_contrib::json::Json;

#[get("/<id>")]
pub async fn get_album_by_id(id: i64, albums: State<'_, AlbumSqliteRepo>) -> Result<Json<Album>, NotFound<String>> {
    albums.find_by_id(id).await
        .map(|a| Json(a))
        .map_err(|_| NotFound(format!("No album with id {} found!", id)))
}