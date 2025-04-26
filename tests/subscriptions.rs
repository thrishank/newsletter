mod helpers;

#[tokio::test]
async fn subscription_works_with_valid_data() {
    let address = helpers::start_server();

    let client = reqwest::Client::new();
    let body = "name=alan%20phile&email=hey_phil%40gmail.com";
    let response = client
        .post(address + "/subscriptions")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 200)
}

// Connection URL: postgresql://postgres:mysecretpassword@localhost:5432/email_newsletter

#[tokio::test]
async fn subscriptions_fails_with_invalid_data() {
    let address = helpers::start_server();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=cheow", "missing email"),
        ("email=leslie%40chow.com", "missing name"),
        (" ", "missing name and missing email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail when the payload is {}",
            error_message
        );
    }
}
