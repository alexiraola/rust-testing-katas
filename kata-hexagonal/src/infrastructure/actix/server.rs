use actix_web::{
    get, middleware, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

use crate::{
    application::{dtos::UserRegisterRequest, user_register_service::UserRegisterService},
    infrastructure::{
        actix::response::ActixHttpResponse, http::HttpRequest,
        in_memory_user_repository::InMemoryUserRepository,
        user_register_controller::UserRegisterController,
    },
};

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

pub async fn create_server(host: &str, port: u16) -> std::io::Result<()> {
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
    .bind((host, port))?
    .run()
    .await
}
