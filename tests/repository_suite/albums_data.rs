use aerodrome_core::model::album::Album;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ALBUM_PEPPERS: Album = Album {
        id: 1,
        name: "Sgt Peppers".to_string(),
        artist: "The Beatles".to_string()
    };

    pub static ref ALBUM_ABBEY: Album = Album {
        id: 2,
        name: "Abbey Road".to_string(),
        artist: "The Beatles".to_string()
    };

    pub static ref ALBUM_RADIOACTIVITY: Album = Album {
        id: 3,
        name: "Radioactivity".to_string(),
        artist: "Kraftwerk".to_string()
    };
}