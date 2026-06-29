#[tokio::test]
async fn health_check_works() {
    // Prepare
    let address = hound::app::run(None, None)
        .expect("Failed to spawn app")
        .to_string();

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("http://127.0.0.1:{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
