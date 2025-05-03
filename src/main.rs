use email_newsletter::config::get_configuration;
use email_newsletter::server;
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to get config");
    let connection =
        PgPoolOptions::new().connect_lazy_with(configuration.database.connect_options());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listner = TcpListener::bind(address).expect("Failed to bind to the listner");

    server(listner, connection)?.await
}
