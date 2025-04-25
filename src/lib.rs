use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, web};

pub fn server() -> Result<Server, std::io::Error> {
    let app = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:3000")?
        .run();
    Ok(app)
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
