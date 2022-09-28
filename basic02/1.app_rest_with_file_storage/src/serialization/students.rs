
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub uid: String,
    pub name: String,
    pub age: u8
}

