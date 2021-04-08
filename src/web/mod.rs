use crate::store::album::AlbumSqliteRepo;

mod aerodrome_api;
mod subsonic_api;

pub struct AerodromeWebServer {
    albums: AlbumSqliteRepo
}

impl AerodromeWebServer {
    pub fn init(albums: AlbumSqliteRepo) -> Self {
        AerodromeWebServer {
            albums
        }
    }

    pub fn run(self) -> rocket::Rocket {
        rocket::ignite()
            .manage(self.albums)
            .mount("/albums", rocket::routes![
                aerodrome_api::albums::get_album_by_id
            ])
    }
}