use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, web};
use std::net::TcpListener;

pub fn server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let app = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/test", web::get())
    })
    .listen(listener)?
    .run();
    Ok(app)
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
