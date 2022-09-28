use actix_web::dev::ServiceRequest;
use crate::errors::JWTError;

pub mod jwt;




pub fn process_token(request: &ServiceRequest) -> Result<String, JWTError> {
    match jwt::HeaderToken::extract_header_token(request) {
        Ok(token) => {
            match jwt::HeaderToken::check_password(token) {
                Ok(token) => Ok(token),
                Err(error) => Err(error)
            }
        },
        Err(error) => Err(error)
    }
}