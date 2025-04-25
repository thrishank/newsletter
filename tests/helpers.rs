use email_newsletter::server;

pub fn start_server() {
    let server = server().expect("Failed to start server");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}
