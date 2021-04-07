use sqlx::{SqlitePool, Database};
use crate::store::album::AlbumRepository;

mod aerodrome_api;
mod subsonic_api;

pub struct AerodromeWebServer<D: Database> {
    albums: AlbumRepository<D>
}

impl <D: Database> AerodromeWebServer<D> {
    pub fn init(albums: AlbumRepository<D>) -> Self {
        AerodromeWebServer {
            albums
        }
    }

    pub fn run(self) -> rocket::Rocket {
        rocket::ignite()
            .manage(self.albums)
            .mount("/hello", rocket::routes![aerodrome_api::world])
    }
}