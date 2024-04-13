mod layouts;
mod models;

use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::str::FromStr;

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web, Postgres, and SQLX";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[post("/api/convert")]
async fn convert_text_handler(text_schema: web::Json<models::TextSchema>) -> impl Responder {
    let from_result = layouts::LayoutCode::from_str(&text_schema.from);
    let to_result = layouts::LayoutCode::from_str(&text_schema.to);

    if let (Ok(from), Ok(to)) = (from_result, to_result) {
        let converted_text = layouts::convert_text(text_schema.text.clone(), from, to);
        HttpResponse::Ok().json(serde_json::json!({"status": "success", "data": converted_text}))
    } else {
        HttpResponse::BadRequest().json(
            serde_json::json!({"status": "error", "message": "Invalid layout codes provided."}),
        )
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(health_checker_handler)
            .service(convert_text_handler)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
