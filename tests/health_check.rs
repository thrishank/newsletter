mod helpers;

#[tokio::test]
async fn health_check_works() {
    let address = helpers::start_server();

    let client = reqwest::Client::new();
    let response = client
        .get(address + "/health_check")
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
