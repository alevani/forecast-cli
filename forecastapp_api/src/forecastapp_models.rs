use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub color: String,
    pub name: String,
}
