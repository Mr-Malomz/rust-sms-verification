use actix_web::{App, HttpServer};
use handlers::{send_otp, verify_otp};

mod handlers;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(send_otp).service(verify_otp))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
