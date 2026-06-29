use hound::infrastructure::database::DevelopmentDatabase;

#[tokio::test]
async fn health_check_works() {
    // Prepare
    let db_pool = DevelopmentDatabase
        .connect()
        .await
        .expect("Failed to connect to database");
    let address = hound::app::run(None, db_pool)
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
