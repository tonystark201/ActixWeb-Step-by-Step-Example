
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Teacher {
    pub uid: String,
    pub name: String,
    pub age: i32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewTeacher{
    pub name: String,
    pub age: i32
}