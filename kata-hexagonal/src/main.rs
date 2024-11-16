mod application;
mod domain;
mod infrastructure;

use infrastructure::actix::server::create_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    create_server("127.0.0.1", 8080).await
}
