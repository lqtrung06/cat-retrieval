use hound::infrastructure::database::DevelopmentDatabase;

#[tokio::test]
async fn session_id_cookie_authenticates_me() {
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
    let login_response = client
        .post(&format!("http://127.0.0.1:{}/login", &address))
        .json(&serde_json::json!({
            "username": "admin",
            "password": "admin"
        }))
        .send()
        .await
        .expect("Failed to execute login request.");

    let session_cookie = login_response
        .headers()
        .get(reqwest::header::SET_COOKIE)
        .expect("login should set a session cookie")
        .to_str()
        .expect("session cookie should be valid header text")
        .split(';')
        .next()
        .expect("session cookie should contain a cookie pair")
        .to_owned();

    let me_response = client
        .get(&format!("http://127.0.0.1:{}/me", &address))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .await
        .expect("Failed to execute me request.");

    // Assert
    assert_eq!(200, login_response.status().as_u16());
    assert_eq!(200, me_response.status().as_u16());

    let body: serde_json::Value = me_response
        .json()
        .await
        .expect("Failed to parse me response body.");
    assert_eq!(body["username"], "admin");
    assert_eq!(body["role"], "admin");
}
