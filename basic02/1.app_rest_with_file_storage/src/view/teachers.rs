use actix_web::{HttpRequest, web, Result, HttpResponse, Responder, HttpMessage};
use actix_web::http::Method;
use log::debug;
use serde::Serialize;
use serde_json::{Map, Value};
use crate::cache::Cache;
use crate::serialization::teachers::Teacher;


pub async fn teachers(req: HttpRequest, teacher: Option<web::Json<Teacher>>)-> Result<impl Responder>{
    let cache = Cache::new(String::from("./teachers.json"));
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
            debug!("Create : {:?}",teacher);
            match teacher {
                Some(teacher)=>{
                    cache.create(
                        &teacher.uid[..],
                        &teacher.name[..],
                        teacher.age
                    );
                    Ok(HttpResponse::Ok().json(teacher.into_inner()))
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

pub async fn teacher(req: HttpRequest,path:web::Path<String>,teacher:Option<web::Json<Teacher>>) -> Result<impl Responder>{
    let cache = Cache::new(String::from("./teachers.json"));
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
            debug!("Put: {:?}",teacher);
            match teacher {
                Some(teacher)=>{
                    cache.update(
                        &teacher.uid[..],
                        &teacher.name[..],
                        teacher.age
                    );
                    Ok(HttpResponse::Ok().json(teacher.into_inner()))
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