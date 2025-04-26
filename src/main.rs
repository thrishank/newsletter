use email_newsletter::server;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:3000").expect("Failed to bind to the listner");
    server(listner)?.await
}
