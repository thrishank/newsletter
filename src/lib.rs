use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, web};
use std::net::TcpListener;
mod config;

pub fn server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let app = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/test", web::get())
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(app)
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
