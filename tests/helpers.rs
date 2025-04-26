use std::net::TcpListener;

use email_newsletter::server;

pub fn start_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to address");
    let port = listener.local_addr().unwrap().port();
    println!("Listening on port {}", port);
    let server = server(listener).expect("Failed to start server");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
