extern crate hmac;
extern crate jwt;
extern crate sha2;

use hmac::{Hmac,Mac};
use jwt::{Header,Token, VerifyWithKey, AlgorithmType, SignWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use actix_http::header::{HeaderValue, ToStrError};
use actix_web::dev::ServiceRequest;
use actix_web::web::to;
use chrono::Utc;
use log::error;
use serde_json::to_string;
use crate::errors::JWTError;

#[derive(Debug)]
pub struct JwtToken {
    pub user_id: String,
    pub body: String
}

impl JwtToken {

    pub fn encode(user_id: &str)->Result<String,JWTError> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret").unwrap();
        let header = Header{
            algorithm: AlgorithmType::Hs256,
            ..Default::default()
        };

        let mut claims = BTreeMap::new();

        let now = &Utc::now().to_string()[..];

        let expiration = &Utc::now()
            .checked_add_signed(chrono::Duration::seconds(60))
            .expect("valid timestamp")
            .to_string()[..];

        claims.insert("iss", "actixweb service");
        claims.insert("sub", "Authentication");
        claims.insert("aud", "User System");
        claims.insert("iat", now);
        claims.insert("exp", expiration);
        claims.insert("user_id", user_id);

        match  Token::new(header, claims).sign_with_key(&key){
            Ok(token) => {
                Ok(String::from(token))
            },
            Err(error) =>{
                Err(JWTError::JWTTokenEncodeError)
            }
        }
    }

    pub fn decode(encoded_token: String)-> Result<JwtToken,JWTError> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret").unwrap();
        let token_str: &str = encoded_token.as_str();
        let result:Result<Token<Header, BTreeMap<String, String>, _>,_> =  token_str.verify_with_key(&key);
        match result {
            Ok(token) =>{
                let _header = token.header();
                let claims = token.claims();
                Ok(JwtToken{
                    user_id: claims.get("user_id").unwrap().to_string(),
                    body: token_str.to_string()
                })
            },
            Err(_) => Err(JWTError::JWTTokenDeocdeError)
        }
    }
}

pub struct HeaderToken{}

impl HeaderToken {

    pub fn extract_header_token(request: &ServiceRequest) -> Result<String, JWTError> {

        fn match_token(token: &HeaderValue)->Result<String,JWTError>{
            match token.to_str() {
                Ok(token) if token.starts_with("BEARER ") => {
                    let token = token[7..].to_string();
                    Ok(token)
                },
                _ => {
                    Err(JWTError::JWTTokenNotFoundError)
                }
            }
        }

        match request.headers().get("AUTHORIZATION") {
            Some(token) => {
                match_token(token)
            },
            None => Err(JWTError::JWTTokenNotFoundError)
        }
    }

    pub fn check_token(password: String) -> Result<String, JWTError> {
        match JwtToken::decode(password) {
            Ok(token) => Ok(token.user_id),
            Err(error) => Err(error)
        }
    }

}


#[cfg(test)]
mod jwt_tests {
    use super::JwtToken;
    use actix_web::test;
    use uuid::Uuid;
    use crate::auth::jwt::HeaderToken;
    use crate::errors::JWTError;
    use crate::errors::JWTError::JWTTokenDeocdeError;

    #[test]
    async fn decode() {
        let user_id = Uuid::new_v4().as_simple().to_string();
        let encoded_token: String = JwtToken::encode(&user_id[..]).unwrap();
        let token = JwtToken::decode(encoded_token).unwrap();
        assert_eq!(user_id,token.user_id);
    }

    #[test]
    async fn decode_error() {
        let encoded_token: String = String::from("Error");
        let error =  JwtToken::decode(encoded_token).unwrap_err();
        assert_eq!(error, JWTError::JWTTokenDeocdeError)
    }

    #[test]
    async fn extract_header_token() {
        let user_id = Uuid::new_v4().as_simple().to_string();
        let encoded_token: String = JwtToken::encode(&user_id[..]).unwrap();
        let token = String::from("BEARER ") + &encoded_token[..];

        let request = test::TestRequest::default()
            .insert_header(("AUTHORIZATION", token))
            .to_srv_request();

        let out_come = HeaderToken::extract_header_token(&request).unwrap();

        let token = JwtToken::decode(out_come).unwrap();
        assert_eq!(user_id,token.user_id);
    }

    #[test]
    async fn extract_header_with_no_token() {

        let request = test::TestRequest::default()
            .insert_header(("Key", "Nothing"))
            .to_srv_request();

        let error = HeaderToken::extract_header_token(&request).unwrap_err();
        assert_eq!(error,JWTError::JWTTokenNotFoundError)

    }

    #[test]
    async fn extract_header_with_wrong_token() {

        let request = test::TestRequest::default()
            .insert_header(("AUTHORIZATION", "Wrong Token"))
            .to_srv_request();

        let error = HeaderToken::extract_header_token(&request).unwrap_err();
        assert_eq!(error,JWTError::JWTTokenNotFoundError)
    }

    #[test]
    async fn check_token() {

        let user_id = Uuid::new_v4().as_simple().to_string();
        let encoded_token: String = JwtToken::encode(&user_id[..]).unwrap();
        let token = String::from("BEARER ") + &encoded_token[..];

        let request = test::TestRequest::default()
            .insert_header(("AUTHORIZATION", token))
            .to_srv_request();

        let token = HeaderToken::extract_header_token(&request).unwrap();
        let result = HeaderToken::check_token(token).unwrap();
        assert_eq!(user_id, result);
    }

    #[test]
    async fn check_wrong_token() {

        let request = test::TestRequest::default()
            .insert_header(("AUTHORIZATION", "BEARER WrongToken"))
            .to_srv_request();

        let token = HeaderToken::extract_header_token(&request).unwrap();
        let error = HeaderToken::check_token(token).unwrap_err();
        assert_eq!(error, JWTTokenDeocdeError);
    }


}

