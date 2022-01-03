use actix_web::http;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};

#[derive(derive_more::Display, Debug)]
pub enum UserError {
    #[display(fmt = "Invalid input parameter")]
    ValidationError,
    #[display(fmt = "Internal server error")]
    InternalError,
    #[display(fmt = "Not found")]
    NotFoundError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({"msg": self.to_string()}))
    }
    fn status_code(&self) -> http::StatusCode {
        match *self {
            UserError::ValidationError => StatusCode::BAD_REQUEST,
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::NotFoundError => StatusCode::NOT_FOUND,
        }
    }
}
