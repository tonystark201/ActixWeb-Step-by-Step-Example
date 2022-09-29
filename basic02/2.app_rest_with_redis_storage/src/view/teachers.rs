use std::collections::HashMap;
use actix_web::{HttpRequest, web, Result, HttpResponse, Responder};
use actix_web::http::Method;
use log::debug;
use crate::{AppData};
use crate::cache::Cache;
use crate::serialization::teachers::Teacher;

pub async fn teachers(req: HttpRequest, teacher: Option<web::Json<Teacher>>,app_data:web::Data<AppData>)-> Result<impl Responder>{
    let pool = app_data.pool.clone();
    match req.method(){
        &Method::GET =>{
            debug!("List teachers");
            let cache: Cache<Teacher> = Cache::new(None,None,pool);
            let data = cache.list("teachers");
            match data {
                Ok(value) => {
                    let mut hm:HashMap<String,Teacher> = HashMap::new();
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
            debug!("Create : {:?}",teacher);
            match teacher {
                Some(teacher)=>{
                    let teacher = teacher.into_inner();
                    let cache = Cache::new(
                        Some(teacher.uid.clone()),
                        Some(teacher.clone()),
                        pool
                    );
                    cache.create("teachers");
                    Ok(HttpResponse::Ok().json(teacher))
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

pub async fn teacher(req: HttpRequest,path:web::Path<String>,teacher:Option<web::Json<Teacher>>,app_data:web::Data<AppData>) -> Result<impl Responder>{
    let pool = app_data.pool.clone();

    match req.method(){
        &Method::GET =>{
            let uid = path.into_inner();
            debug!("GET: {}",&uid);
            let cache: Cache<Teacher> = Cache::new(Some(uid),None,pool);
            let data = cache.get("teachers")
                .unwrap_or_else(|_| {String::new()});
            let data:Teacher = serde_json::from_str(&data).unwrap();
            let response = HttpResponse::Ok().json(data);
            Ok(response)
        }
        &Method::DELETE=>{
            let uid = path.into_inner();
            debug!("DELETE: {}",&uid);
            let cache: Cache<Teacher> = Cache::new(Some(uid),None,pool);
            cache.delete("teachers");
            Ok(HttpResponse::NoContent().finish())
        },
        &Method::PUT=>{
            let uid = path.into_inner();
            debug!("PUT: {}",&uid);
            match teacher {
                Some(teacher)=>{
                    let teacher = teacher.into_inner();
                    let cache = Cache::new(
                        Some(teacher.uid.clone()),
                        Some(teacher.clone()),
                        pool
                    );
                    cache.update("teachers");
                    Ok(HttpResponse::Ok().json(teacher))
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