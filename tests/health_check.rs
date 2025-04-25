mod helpers;

#[tokio::test]
async fn health_check_works() {
    helpers::start_server();
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:3000/health_check")
        .send()
        .await
        .expect("Failed to send request");
    println!("Response: {:?}", response.status().is_success());
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
