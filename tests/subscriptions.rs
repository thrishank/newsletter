mod helpers;

#[tokio::test]
async fn subscription_works_with_valid_data() {
    let server = helpers::start_server().await;

    let client = reqwest::Client::new();

    let body = "name=alan%20phil&email=hey_phil%40hangover.com";
    let response = client
        .post(server.address + "/subscriptions")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&server.connection)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "hey_phil@hangover.com");
    assert_eq!(saved.name, "alan phil");

    println!("{:?}", saved)
}

// Connection URL: postgresql://postgres:mysecretpassword@localhost:5432/email_newsletter

#[tokio::test]
async fn subscriptions_fails_with_invalid_data() {
    let address = helpers::start_server().await.address;
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
