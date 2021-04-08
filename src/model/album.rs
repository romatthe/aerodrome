use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Album {
    pub id: i64,
    pub name: String,
    pub artist: String,
}