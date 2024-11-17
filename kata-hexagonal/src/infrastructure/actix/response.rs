use std::{error::Error, fmt::Display};

use actix_web::{http::StatusCode, HttpResponse};

use crate::infrastructure::http;

pub struct ActixHttpResponse<T> {
    status: Option<StatusCode>,
    data: Option<Result<T, Box<dyn Error>>>,
}

impl<T: Display> ActixHttpResponse<T> {
    pub fn new() -> Self {
        ActixHttpResponse {
            status: None,
            data: None,
        }
    }

    pub fn response(&self) -> HttpResponse {
        match (self.status, &self.data) {
            (Some(status), Some(Ok(data))) => HttpResponse::build(status).json(format!("{}", data)),
            (Some(status), Some(Err(error))) => HttpResponse::build(status).json(error.to_string()),
            (Some(status), None) => HttpResponse::new(status),
            _other => HttpResponse::InternalServerError().body("Unknown error".to_string()),
        }
    }
}

impl<T> http::HttpResponse<Result<T, Box<dyn Error>>> for ActixHttpResponse<T> {
    fn status(&mut self, code: u16) -> &mut Self {
        if let Ok(status) = StatusCode::from_u16(code) {
            self.status = Some(status);
        }
        self
    }

    fn json(&mut self, data: Result<T, Box<dyn Error>>) -> &mut Self {
        self.data = Some(data);
        self
    }
}
