use rocket::{get, State};
use sqlx::SqlitePool;

#[get("/")]
pub fn world() -> &'static str {
    "hello, world!"
}

#[get("/")]
pub fn state(pool: State<SqlitePool>) -> &'static str {
    todo!("Use the state somehow")
}