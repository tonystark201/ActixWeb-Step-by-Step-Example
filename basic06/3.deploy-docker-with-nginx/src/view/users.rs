use actix_web::{HttpRequest, HttpResponse, Responder, web, Result};
use actix_web::http::Method;
use log::debug;
use crate::AppData;
use crate::dao::db::PgUser;
use crate::serialization::users::NewUser;

pub async fn users(req: HttpRequest, user: Option<web::Json<NewUser>>, app_data:web::Data<AppData>) -> Result<impl Responder>{
    let db_conn = &app_data.db_conn;
    let pg = PgUser::new(db_conn).unwrap();
    match req.method(){
        &Method::POST=>{
            debug!("Create : {:?}",user);
            match user {
                Some(user)=>{
                    let user = user.into_inner();
                    match pg.create(&user.name[..],&user.email[..],&user.password[..]){
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