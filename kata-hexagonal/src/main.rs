mod application;
mod domain;
mod infrastructure;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use application::{dtos::UserRegisterRequest, user_register_service::UserRegisterService};
use infrastructure::{
    http::HttpRequest, in_memory_user_repository::InMemoryUserRepository,
    user_register_controller::UserRegisterController,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    email: String,
    password: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/register")]
async fn register(form: web::Json<FormData>) -> impl Responder {
    let mut repo = InMemoryUserRepository::new();
    let mut service = UserRegisterService::new(&mut repo);
    let mut controller = UserRegisterController::new(&mut service);
    let request = HttpRequest {
        body: UserRegisterRequest {
            email: form.email.clone(),
            password: form.password.clone(),
        },
    };
    let mut response = infrastructure::http::HttpResponse::new();

    controller.register(request, &mut response).await;

    if let Some(data) = response.data {
        match data {
            Ok(d) => HttpResponse::Created().json(format!("id: {}, email: {}", d.id, d.email)),
            Err(error) => HttpResponse::NotFound().json(error.to_string()),
        }
    } else {
        HttpResponse::NotFound().json("Not found".to_string())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(register))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
