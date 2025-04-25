use email_newsletter::server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    server()?.await
}
