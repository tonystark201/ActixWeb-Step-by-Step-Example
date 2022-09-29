use actix_web::http::{Method, StatusCode};
use actix_web::{HttpResponse, Responder, web, Result, HttpRequest};
use log::debug;
use crate::AppData;
use crate::auth::jwt::JwtToken;
use crate::dao::db::PgUser;
use crate::serialization::users::Login;

const LOGIN: &'static str = r#"
<!doctype html>
<html>
    <head>
        <title>Actix Web</title>
    </head>
    <body>
        <h1>V1:Login</h>
        <p>Hello, welcome to actix web<p>
    </body>
</html>
"#;

pub async fn login_placeholder() -> HttpResponse {
    let res = HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(LOGIN);
    res
}

pub async fn login(req: HttpRequest,credentials: web::Json<Login>,app_data:web::Data<AppData>) -> Result<impl Responder> {
    let db_conn = &app_data.db_conn;
    let pg = PgUser::new(db_conn).unwrap();

    match req.method(){
        &Method::POST =>{
            let username: String = credentials.username.clone();
            let password: String = credentials.password.clone();
            let user = pg.retrieve_by_username(&username[..]);
            match user {
                Some(user)  => {
                    match user.verify(password){
                        true => {
                            let token: String = JwtToken::encode(&user.uid[..]).unwrap();
                            debug!("Generate JWT Token: {:?}",token);
                            Ok(HttpResponse::Ok().insert_header(("token", token)).finish())
                        },
                        false => {
                            Ok(HttpResponse::Unauthorized().finish())
                        }
                    }
                },
                None => {
                    Ok(HttpResponse::Unauthorized().finish())
                }
            }
        },
        _ =>{
            Ok(HttpResponse::MethodNotAllowed().finish())
        }
    }
}
