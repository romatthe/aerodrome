use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist: String,
}
