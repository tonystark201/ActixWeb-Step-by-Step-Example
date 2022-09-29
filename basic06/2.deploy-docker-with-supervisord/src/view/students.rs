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

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;
    use std::sync::Arc;
    use actix_http::body::to_bytes;
    use actix_web::{test, web, App};
    use actix_web::http::Method;
    use actix_web::web::Bytes;
    use diesel::{Connection, ConnectionError, PgConnection};
    use log::error;
    use r2d2::{Error, Pool};
    use r2d2_redis::RedisConnectionManager;
    use crate::AppData;
    use crate::models::{Student};
    use crate::serialization::students::NewStudent;
    use crate::view::students::{student, students};

    fn get_database_url() -> String {
        dotenv().ok();
        let db_addr= env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:15432/postgres".into())
            .parse::<String>()
            .unwrap();
        db_addr
    }

    fn get_redis_url() -> String {
        dotenv().ok();
        let redis_addr= env::var("REDIS")
            .unwrap_or_else(|_| "redis://localhost:16379/1".into())
            .parse::<String>()
            .unwrap();
        redis_addr
    }

    fn db_connection(database_url: &str) -> Result<PgConnection, ConnectionError> {
        match PgConnection::establish(&database_url) {
            Ok(value) => Ok(value),
            Err(e) => {
                error!("Error connecting to {}", database_url);
                Err(e)
            }
        }
    }

    fn redis_pool(redis_url:&str) -> Result<Pool<RedisConnectionManager>, Error> {
        let manager = RedisConnectionManager::new(redis_url).unwrap();
        let pool = r2d2::Pool::new(manager);
        pool
    }

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }


    #[actix_web::test]
    async fn test_students() {

        let db_url = get_database_url();
        let redis_url = get_redis_url();
        let conn = db_connection(&db_url).unwrap();
        let pool = redis_pool(&redis_url).unwrap();

        let app_data = web::Data::new(
            AppData{
                pool: Arc::new(pool),
                db_conn: conn
            }
        );

        let app = test::init_service(
            App::new().app_data(app_data)
                .route("/students", web::to(students))
                .route("/student/{uid}",web::to(student))
        ).await;

        // Test Post
        let req = test::TestRequest::post()
            .insert_header(("Content-Type","application/json"))
            .set_json(
                NewStudent{
                    name: String::from("test_student_one"),
                    age: 20
                }
            )
            .uri("/students")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        // get the created student
        let body = to_bytes(resp.into_body()).await.unwrap();
        let student: Student = serde_json::from_str(body.as_str()).unwrap();

        // Test List
        let req = test::TestRequest::get()
            .insert_header(("Content-Type","application/json"))
            .uri("/students")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Test Get
        let uri = format!("/student/{}",student.uid);
        let req = test::TestRequest::get()
            .insert_header(("Content-Type","application/json"))
            .uri(&uri[..])
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Test PUT
        let uri = format!("/student/{}",student.uid);
        let req = test::TestRequest::put()
            .insert_header(("Content-Type","application/json"))
            .set_json(
                NewStudent{
                    name: "TestUpdatedName-one".to_string(),
                    age:20
                }
            )
            .uri(&uri[..])
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Test DELETE
        let uri = format!("/student/{}",student.uid);
        let req = test::TestRequest::delete()
            .insert_header(("Content-Type","application/json"))
            .uri(&uri[..])
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}