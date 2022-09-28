use actix_files::NamedFile;
use actix_http::{Method, StatusCode};
use actix_web::{get, http::{
    header::ContentType,
}, cookie::Cookie, middleware, web, App, HttpResponse, HttpServer, Responder, Result, Either, HttpRequest};
use log::{debug, info};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Person {
    uid: i32,
    name: String,
    age: i32,
}


#[get("/index")]
async fn index() -> Result<impl Responder> {
    Ok(NamedFile::open("static/index.html")?)
}

async fn persons(req: HttpRequest,person:Option<web::Json<Person>>)-> Result<impl Responder>{
    match req.method(){
        &Method::GET =>{
            debug!("Get person list");
            let mut list: Vec<Person> = Vec::new();
            list.push(Person{
                uid:1,
                name:"James".to_string(),
                age: 20,
            });
            list.push(Person{
                uid:2,
                name:"Alice".to_string(),
                age: 30,
            });
            let string = serde_json::to_string(&list).unwrap();
            let response = HttpResponse::Ok()
                .content_type("application/json")
                .body(string);

            Ok(response)

        }
        &Method::POST=>{
            debug!("Create person: {:?}",person);
            Ok(HttpResponse::Ok().json(person))
        },
        _ =>{
            debug!("Not Found");
            Ok(HttpResponse::Found().header("Location", "/v1/404").finish())
        }
    }
}

async fn person(req: HttpRequest,path:web::Path<i32>,person:Option<web::Json<Person>>)-> Result<impl Responder>{
    match req.method(){
        &Method::GET =>{
            debug!("Retrieve person");
            let uid = path.into_inner();
            let person = Person{
                uid: uid,
                name:"James".to_string(),
                age: 20,
            };
            let response = HttpResponse::Created().json(person);
            Ok(response)
        }
        &Method::DELETE=>{
            debug!("Delete person");
            let uid = path.into_inner();
            Ok(HttpResponse::NoContent().finish())
        },
        &Method::PUT=>{
            debug!("Put person: {:?}",person);
            Ok(HttpResponse::Ok().json(person))
        },
        _ =>{
            debug!("Not Found");
            Ok(HttpResponse::Found().header("Location", "/v1/404").finish())
        }
    }
}

async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("static/404.html")?
                .customize()
                .with_status(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish()))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("v1")
                    .service(index)
                    .service(web::resource("/persons").route(web::to(persons)))
                    .service(web::resource("/person/{uid}").route(web::to(person)))
            )
            .default_service(web::to(default_handler))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}