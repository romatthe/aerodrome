use crate::store::album::AlbumSqliteRepo;
use crate::store::track::TrackSqliteRepo;

mod aerodrome_api;
mod subsonic_api;

pub struct AerodromeWebServer {
    albums: AlbumSqliteRepo,
    tracks: TrackSqliteRepo,
}

impl AerodromeWebServer {
    pub fn init(albums: AlbumSqliteRepo, tracks: TrackSqliteRepo) -> Self {
        AerodromeWebServer { albums, tracks }
    }

    pub fn run(self) -> rocket::Rocket {
        rocket::ignite()
            .manage(self.albums)
            .manage(self.tracks)
            .mount(
                "/albums",
                rocket::routes![aerodrome_api::albums::get_album_by_id],
            )
            .mount(
                "/tracks",
                rocket::routes![aerodrome_api::tracks::get_track_by_id],
            )
            .mount("/rest", rocket::routes![
                // System
                subsonic_api::system::ping,
                subsonic_api::system::get_license,
                // Browsing
                subsonic_api::browsing::get_song,
            ])
    }
}
