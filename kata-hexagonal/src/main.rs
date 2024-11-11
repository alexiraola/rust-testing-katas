mod application;
mod domain;
mod infrastructure;

use actix_web::{
    get, middleware, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use application::{dtos::UserRegisterRequest, user_register_service::UserRegisterService};
use infrastructure::{
    actix::response::ActixHttpResponse, http::HttpRequest,
    in_memory_user_repository::InMemoryUserRepository,
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
async fn register(repo: Data<InMemoryUserRepository>, form: web::Json<FormData>) -> impl Responder {
    let service = UserRegisterService::new(repo.into_inner());
    let mut controller = UserRegisterController::new(service);
    let request = HttpRequest {
        body: UserRegisterRequest {
            email: form.email.clone(),
            password: form.password.clone(),
        },
    };
    let mut response = ActixHttpResponse::new();

    controller.register(request, &mut response).await;

    response.response()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let repo = Data::new(InMemoryUserRepository::new());

    HttpServer::new(move || {
        App::new()
            .app_data(repo.clone())
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
