use shared::model::resp::{ApiResponse, Status};

use actix_web::HttpResponse;
use serde::Serialize;

pub trait TApiResponse<T: Serialize> {
    fn new(status: Status, message: Option<String>, data: Option<T>) -> Self;
    fn to_http_response(&self) -> HttpResponse;
}

impl<T: Serialize> TApiResponse<T> for ApiResponse<T> {
    fn new(status: Status, message: Option<String>, data: Option<T>) -> Self {
        ApiResponse {
            status,
            message,
            data,
        }
    }

    // Create an HttpResponse from ApiResponse
    fn to_http_response(&self) -> HttpResponse {
        match self.status {
            Status::Success => HttpResponse::Ok().json(self),
            Status::BadRequest => HttpResponse::BadRequest().json(self),
        }
    }
}

// Utility functions for common responses
pub fn success<T: Serialize>(data: T) -> HttpResponse {
    ApiResponse::new(Status::Success, None, Some(data)).to_http_response()
}

pub fn ok() -> HttpResponse {
    ApiResponse::new(Status::Success, None, Option::<()>::None).to_http_response()
}

pub fn error(message: String) -> HttpResponse {
    ApiResponse::new(Status::BadRequest, Some(message), Option::<()>::None).to_http_response()
}
