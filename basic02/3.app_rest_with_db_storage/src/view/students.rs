use std::collections::HashMap;
use actix_web::{HttpRequest, web, Result, HttpResponse, Responder};
use actix_web::http::Method;
use diesel::result::Error;
use log::debug;
use crate::{AppData};
use crate::dao::cache::Cache;
use crate::dao::db::PgStudent;
use crate::serialization::students::{NewStudent, Student};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StudentRequest {
    limit:Option<i64>,
    offset:Option<i64>
}

pub async fn students(req: HttpRequest, param:web::Query<StudentRequest>, student: Option<web::Json<NewStudent>>,app_data:web::Data<AppData>)-> Result<impl Responder>{
    let db_conn = &app_data.db_conn;
    let pg = PgStudent::new(db_conn).unwrap();
    match req.method(){
        &Method::GET =>{
            debug!("List students");
            let offset = param.offset.unwrap_or(0);
            let limit = param.limit.unwrap_or(10);
            let data = pg.list(limit,offset);
            match data {
                Some(value) => {
                    Ok(HttpResponse::Ok().json(value))
                },
                None=>{
                    Ok(HttpResponse::BadRequest().json({}))
                }
            }
        }
        &Method::POST=>{
            debug!("Create : {:?}",student);
            match student {
                Some(student)=>{
                    let student = student.into_inner();
                    match pg.create(&student.name[..],student.age){
                        Ok(value) => {
                            Ok(HttpResponse::Ok().json(value))
                        },
                        Err(error)=>{
                            Ok(HttpResponse::InternalServerError().json(format!("{:?}",error)))
                        }
                    }
                },
                None => {
                    Ok(HttpResponse::BadRequest().json({}))
                }
            }
        },
        _ =>{
            debug!("Not Found");
            Ok(HttpResponse::Found().header("Location", "/v1/404").finish())
        }
    }
}

pub async fn student(req: HttpRequest,path:web::Path<i32>,student:Option<web::Json<NewStudent>>,app_data:web::Data<AppData>) -> Result<impl Responder>{
    let db_conn = &app_data.db_conn;
    let pg = PgStudent::new(db_conn).unwrap();
    match req.method(){
        &Method::GET =>{
            let uid = path.into_inner();
            debug!("GET: {}",uid);
            match pg.retrieve(uid){
                Some(value) => {
                    Ok(HttpResponse::Ok().json(value))
                },
                None=>{
                    Ok(HttpResponse::Ok().json({}))
                }
            }
        },
        &Method::DELETE=>{
            let uid = path.into_inner();
            debug!("DELETE: {}",uid);
            match pg.delete(uid){
                Ok(_) => {
                    Ok(HttpResponse::NoContent().finish())
                },
                Err(error) =>{
                    Ok(HttpResponse::BadRequest().json(format!("{:?}",error)))
                }
            }
        },
        &Method::PUT=>{
            let uid = path.into_inner();
            debug!("PUT: {}",uid);
            match student {
                Some(student)=>{
                    let student = student.into_inner();
                    match pg.update(uid,&student.name[..],student.age){
                        Ok(Some(value))=>{
                            Ok(HttpResponse::Ok().json(value))
                        },
                        Ok(None) => {
                            Ok(HttpResponse::Ok().json({}))
                        }
                        Err(error)=>{
                            Ok(HttpResponse::BadRequest().json(format!("{:?}",error)))
                        }
                    }
                },
                None=>{
                    Ok(HttpResponse::BadRequest().json({}))
                }
            }
        },
        _ =>{
            debug!("Not Found");
            Ok(HttpResponse::Found().header("Location", "/v1/404").finish())
        }
    }
}