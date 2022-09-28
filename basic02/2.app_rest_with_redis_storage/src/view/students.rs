use std::collections::HashMap;
use actix_web::{HttpRequest, web, Result, HttpResponse, Responder};
use actix_web::http::Method;
use log::debug;
use crate::{AppData};
use crate::cache::Cache;
use crate::serialization::students::Student;

pub async fn students(req: HttpRequest, student: Option<web::Json<Student>>,app_data:web::Data<AppData>)-> Result<impl Responder>{
    let pool = app_data.pool.clone();
    match req.method(){
        &Method::GET =>{
            debug!("List students");
            let cache: Cache<Student> = Cache::new(None,None,pool);
            let data = cache.list("students");
            match data {
                Ok(value) => {
                    let mut hm:HashMap<String,Student> = HashMap::new();
                    for item in value{
                        hm.insert(
                            item.0,
                            serde_json::from_str(&item.1).unwrap()
                        );
                    }
                    Ok(HttpResponse::Ok().json(hm))
                },
                Err(error)=>{
                    Ok(HttpResponse::BadRequest().json(format!("{:?}",error)))
                }
            }
        }
        &Method::POST=>{
            debug!("Create : {:?}",student);
            match student {
                Some(student)=>{
                    let student = student.into_inner();
                    let cache = Cache::new(
                        Some(student.uid.clone()),
                        Some(student.clone()),
                        pool
                    );
                    cache.create("students");
                    Ok(HttpResponse::Ok().json(student))
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

pub async fn student(req: HttpRequest,path:web::Path<String>,student:Option<web::Json<Student>>,app_data:web::Data<AppData>) -> Result<impl Responder>{
    let pool = app_data.pool.clone();

    match req.method(){
        &Method::GET =>{
            let uid = path.into_inner();
            debug!("GET: {}",&uid);
            let cache: Cache<Student> = Cache::new(Some(uid),None,pool);
            let data = cache.get("students")
                .unwrap_or_else(|_| {String::new()});
            let data:Student = serde_json::from_str(&data).unwrap();
            let response = HttpResponse::Ok().json(data);
            Ok(response)
        }
        &Method::DELETE=>{
            let uid = path.into_inner();
            debug!("DELETE: {}",&uid);
            let cache: Cache<Student> = Cache::new(Some(uid),None,pool);
            let data = cache.delete("students");
            Ok(HttpResponse::NoContent().finish())
        },
        &Method::PUT=>{
            let uid = path.into_inner();
            debug!("PUT: {}",&uid);
            match student {
                Some(student)=>{
                    let student = student.into_inner();
                    let cache = Cache::new(
                        Some(student.uid.clone()),
                        Some(student.clone()),
                        pool
                    );
                    cache.update("students");
                    Ok(HttpResponse::Ok().json(student))
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