use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, middleware::Logger, web,
};
use sqlx::PgPool;
use std::net::TcpListener;
pub mod config;
pub mod routes;

pub fn server(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let app = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(app)
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
