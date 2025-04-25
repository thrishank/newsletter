use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

async fn greet() -> impl Responder {
    "Hello, World!"
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
