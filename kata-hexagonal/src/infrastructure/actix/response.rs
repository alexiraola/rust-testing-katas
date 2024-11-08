use std::error::Error;

use actix_web::{http::StatusCode, HttpResponse};

use crate::application::dtos::UserRegisterResponse;
use crate::infrastructure::http;

pub struct ActixHttpResponse {
    status: Option<StatusCode>,
    data: Option<Result<UserRegisterResponse, Box<dyn Error>>>,
}

impl ActixHttpResponse {
    pub fn new() -> Self {
        ActixHttpResponse {
            status: None,
            data: None,
        }
    }

    pub fn response(&self) -> HttpResponse {
        match (self.status, &self.data) {
            (Some(status), Some(Ok(data))) => {
                HttpResponse::build(status).json(format!("id: {}, email: {}", data.id, data.email))
            }
            (Some(status), Some(Err(error))) => HttpResponse::build(status).json(error.to_string()),
            (Some(status), None) => HttpResponse::new(status),
            other => HttpResponse::InternalServerError().body("Unknown error".to_string()),
        }
    }
}

impl http::HttpResponse<Result<UserRegisterResponse, Box<dyn Error>>> for ActixHttpResponse {
    fn status(&mut self, code: u16) -> &mut Self {
        if let Ok(status) = StatusCode::from_u16(code) {
            self.status = Some(status);
        }
        self
    }

    fn json(&mut self, data: Result<UserRegisterResponse, Box<dyn Error>>) -> &mut Self {
        self.data = Some(data);
        self
    }
}
