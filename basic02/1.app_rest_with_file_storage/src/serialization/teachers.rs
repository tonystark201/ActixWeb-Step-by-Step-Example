
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher {
    pub uid: String,
    pub name: String,
    pub age: u8
}