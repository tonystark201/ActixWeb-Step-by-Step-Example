extern crate hmac;
extern crate jwt;
extern crate sha2;

use hmac::{Hmac,Mac};
use jwt::{Header,Token, VerifyWithKey, AlgorithmType, SignWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use actix_http::header::ToStrError;
use actix_web::dev::ServiceRequest;
use chrono::Utc;
use log::error;
use crate::errors::JWTError;

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
        match request.headers().get("AUTHORIZATION") {
            Some(token) => {
                match token.to_str() {
                    Ok(token) if token.starts_with("BEARER ") => {
                        Ok(token.to_string())
                    },
                    _ => {
                        Err(JWTError::JWTTokenNotFoundError)
                    }
                }
            },
            None => Err(JWTError::JWTTokenNotFoundError)
        }
    }

    pub fn check_password(password: String) -> Result<String, JWTError> {
        match JwtToken::decode(password) {
            Ok(token) => Ok(token.user_id),
            Err(error) => Err(error)
        }
    }

}


