use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_check).service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
