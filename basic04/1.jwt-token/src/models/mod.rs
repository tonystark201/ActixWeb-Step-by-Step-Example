extern crate bcrypt;
extern crate diesel;

use chrono::prelude::*;
use diesel::prelude::*;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::schema::{students,teachers,users};
use bcrypt::verify;

pub type Id = i32;

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

#[derive(Queryable,Insertable, Clone, Serialize, Deserialize)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub uid: String,
    pub name: String,
    pub email: String,
    pub password: String
}