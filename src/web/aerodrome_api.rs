use rocket::{get, State};
use sqlx::{SqlitePool, Database, Pool};
use crate::store::album::AlbumRepository;

#[get("/")]
pub fn world() -> &'static str {
    "hello, world!"
}

// #[get("/")]
// pub fn state(pool: State<SqlitePool>) -> &'static str {
//     todo!("Use the state somehow")
// }

#[get("/")]
pub fn state(pool: State<Box<Pool<Box<dyn Database>>>>) -> &'static str {
    todo!("Use the state somehow")
}