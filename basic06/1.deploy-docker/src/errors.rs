use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum JWTError {
    #[error("jwt token not found")]
    JWTTokenNotFoundError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token encode error")]
    JWTTokenEncodeError,
    #[error("jwt token decode error")]
    JWTTokenDeocdeError,
}
