use std::hash::Hash;
use chrono::prelude::*;
use diesel::prelude::*;

use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use serde::{Deserialize, Serialize};
use crate::schema::{students,teachers};

pub type Id = i32;
pub type AGE = i32;

#[derive(Debug,Insertable, Queryable,Serialize, Deserialize)]
#[table_name = "students"]
pub struct Student {
    pub uid: Id,
    pub name: String,
    pub age: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug,Insertable, Queryable,Serialize, Deserialize)]
#[table_name = "teachers"]
pub struct Teacher {
    pub uid: Id,
    pub name: String,
    pub age: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}