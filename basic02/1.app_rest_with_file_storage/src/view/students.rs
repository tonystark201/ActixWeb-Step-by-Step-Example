use actix_web::{HttpRequest, web, Result, HttpResponse, Responder, HttpMessage};
use actix_web::http::Method;
use log::debug;
use serde::Serialize;
use serde_json::{Map, Value};
use crate::cache::Cache;
use crate::serialization::students::Student;

pub async fn students(req: HttpRequest, student: Option<web::Json<Student>>)-> Result<impl Responder>{
    let cache = Cache::new(String::from("./students.json"));
    match req.method(){
        &Method::GET =>{
            debug!("List students");
            let data = cache.list();
            let string = serde_json::to_string(&data).unwrap();
            let response = HttpResponse::Ok()
                .content_type("application/json")
                .body(string);
            Ok(response)
        }
        &Method::POST=>{
            debug!("Create : {:?}",student);
            match student {
                Some(student)=>{
                    cache.create(
                        &student.uid[..],
                        &student.name[..],
                        student.age
                    );
                    Ok(HttpResponse::Ok().json(student.into_inner()))
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

pub async fn student(req: HttpRequest,path:web::Path<String>,student:Option<web::Json<Student>>) -> Result<impl Responder>{
    let cache = Cache::new(String::from("./students.json"));
    match req.method(){
        &Method::GET =>{
            let uid = path.into_inner();
            debug!("GET: {}",&uid);
            let data = cache.get(&uid[..]);
            let response = HttpResponse::Ok().json(data);
            Ok(response)
        }
        &Method::DELETE=>{
            let uid = path.into_inner();
            debug!("DELETE: {}",&uid);
            cache.delete(&uid[..]);
            Ok(HttpResponse::NoContent().finish())
        },
        &Method::PUT=>{
            let uid = path.into_inner();
            debug!("PUT: {}",&uid);
            debug!("Put: {:?}",student);
            match student {
                Some(student)=>{
                    cache.update(
                        &student.uid[..],
                        &student.name[..],
                        student.age
                    );
                    Ok(HttpResponse::Ok().json(student.into_inner()))
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