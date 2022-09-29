use std::sync::Arc;
use bcrypt::{DEFAULT_COST, hash};
use diesel::{Connection, insert_into, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, ExpressionMethods, EqAll};
use diesel::pg::Pg;
use failure::{Error, Fail, format_err};
use uuid::Uuid;
use crate::models::{Student, Teacher,User};
use crate::schema::{students,teachers,users};


pub struct PgStudent<'a> {
    pub conn: &'a PgConnection
}

impl<'a> PgStudent<'a>{

    pub fn new(conn: &'a PgConnection) -> Result<Self,Error>{
        Ok(PgStudent { conn:conn })
    }

    pub fn list(&self, limit:i64, offset:i64) -> Option<Vec<Student>> {
        // use crate::schema::students::dsl::*;
        let res = students::table.select(
                (
                    students::uid,
                    students::name,
                    students::age,
                    students::created_at,
                    students::updated_at
                )
            ).order(students::uid)
            .limit(limit)
            .offset(offset)
            .load(self.conn)
            .optional().unwrap();
        res
    }

    pub fn create(&self,name:&str,age:i32) -> Result<Student, Error> {
        let res = insert_into(students::table)
            .values((
                students::name.eq(name),
                students::age.eq(age),
            ))
            .returning((
                students::uid,
                students::name,
                students::age,
                students::created_at,
                students::updated_at
            ))
            .get_result(self.conn)
            .map_err(Error::from);
        res
    }

    pub fn retrieve(&self,uid:i32)->Option<Student>{
        let res = students::table.filter(students::uid.eq(uid))
            .select(
                (
                    students::uid,
                    students::name,
                    students::age,
                    students::created_at,
                    students::updated_at
                )
            ).first(self.conn).optional().unwrap();
        res
    }

    pub fn update(&self,uid:i32,name:&str, age:i32) -> Result<Option<Student>, diesel::result::Error> {
        let res = diesel::update(
            students::table.filter(students::uid.eq(uid))
        ).set(
                (
                    students::name.eq(name),
                    students::age.eq(age),
                )
            ).returning(
                (
                    students::uid,
                    students::name,
                    students::age,
                    students::created_at,
                    students::updated_at
                )
            ).get_result(self.conn)
            .optional();
        res
    }

    pub fn delete(&self, uid:i32) -> Result<Option<Student>, diesel::result::Error> {
        let res = diesel::delete(students::table)
            .filter(students::uid.eq(uid))
            .returning(
                (
                    students::uid,
                    students::name,
                    students::age,
                    students::created_at,
                    students::updated_at
                )
            )
            .get_result(self.conn)
            .optional();
        res
    }
}

pub struct PgTeacher<'a> {
    pub conn: &'a PgConnection
}

impl<'a> PgTeacher<'a>{

    pub fn new(conn: &'a PgConnection) -> Result<Self,Error>{
        Ok(PgTeacher { conn:conn })
    }

    pub fn list(&self, limit:i64, offset:i64) -> Option<Vec<Teacher>> {
        // use crate::schema::teachers::dsl::*;
        let res = teachers::table.select(
            (
                teachers::uid,
                teachers::name,
                teachers::age,
                teachers::created_at,
                teachers::updated_at
            )
        ).order(teachers::uid)
            .limit(limit)
            .offset(offset)
            .load(self.conn)
            .optional().unwrap();
        res
    }

    pub fn create(&self,name:&str,age:i32) -> Result<Teacher, Error> {
        let res = insert_into(teachers::table)
            .values((
                teachers::name.eq(name),
                teachers::age.eq(age),
            ))
            .returning((
                teachers::uid,
                teachers::name,
                teachers::age,
                teachers::created_at,
                teachers::updated_at
            ))
            .get_result(self.conn)
            .map_err(Error::from);
        res
    }

    pub fn retrieve(&self,uid:i32)->Option<Teacher>{
        let res = teachers::table.filter(teachers::uid.eq(uid))
            .select(
                (
                    teachers::uid,
                    teachers::name,
                    teachers::age,
                    teachers::created_at,
                    teachers::updated_at
                )
            ).first(self.conn).optional().unwrap();
        res
    }

    pub fn update(&self, uid:i32,name:&str, age:i32) -> Result<Option<Teacher>, diesel::result::Error> {
        let res = diesel::update(
            teachers::table.filter(teachers::uid.eq(uid))
        ).set(
            (
                teachers::name.eq(name),
                teachers::age.eq(age),
            )
        ).returning(
            (
                teachers::uid,
                teachers::name,
                teachers::age,
                teachers::created_at,
                teachers::updated_at
            )
        ).get_result(self.conn)
            .optional();
        res
    }

    pub fn delete(&self, uid:i32) -> Result<Option<Teacher>, diesel::result::Error> {
        let res = diesel::delete(teachers::table)
            .filter(teachers::uid.eq(uid))
            .returning(
                (
                    teachers::uid,
                    teachers::name,
                    teachers::age,
                    teachers::created_at,
                    teachers::updated_at
                )
            )
            .get_result(self.conn)
            .optional();
        res
    }
}


pub struct PgUser<'a> {
    pub conn: &'a PgConnection
}

impl<'a> PgUser<'a> {

    pub fn new(conn: &'a PgConnection) -> Result<Self,Error>{
        Ok(PgUser { conn:conn })
    }

    pub fn create(&self,name:&str,email:&str,password:&str) -> Result<User, Error> {
        let hashed_password: String = hash(password, DEFAULT_COST).unwrap();
        let uid = Uuid::new_v4().as_simple().to_string();
        let res = insert_into(users::table)
            .values((
                users::uid.eq(uid),
                users::name.eq(name),
                users::email.eq(email),
                users::password.eq(hashed_password)
            ))
            .returning((
                users::id,
                users::uid,
                users::name,
                users::email,
                users::password
            ))
            .get_result(self.conn)
            .map_err(Error::from);
        res
    }
}