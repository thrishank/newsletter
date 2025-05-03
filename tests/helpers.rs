use email_newsletter::{config::get_configuration, server};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub connection: PgPool,
}

pub async fn start_server() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to address");
    let port = listener.local_addr().unwrap().port();
    println!("Listening on port {}", port);

    let connection = configure_database().await;
    let server = server(listener, connection.clone()).expect("Failed to start server");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);
    TestApp {
        address,
        connection,
    }
}

pub async fn configure_database() -> PgPool {
    let mut configuration = get_configuration().expect("Failed to get config");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let mut connection =
        PgConnection::connect(&configuration.database.connection_string_without_db())
            .await
            .expect("failed to connect to postgres");

    // Create the database
    connection
        .execute(
            format!(
                r#"CREATE DATABASE "{}";"#,
                configuration.database.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to create database");

    // Migrate the database
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("faild to apply migrations");
    connection_pool
}
