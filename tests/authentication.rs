use hound::infrastructure::database::Database;
use uuid::Uuid;

#[tokio::test]
async fn admin_can_login_and_get_me() {
    // Prepare
    let database_name = format!("hound_test_{}", Uuid::new_v4().simple());

    let db_pool = Database
        .clone(&database_name)
        .await
        .expect("Failed to clone test database");

    let address = hound::app::run(None, Some(db_pool))
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

    Database
        .drop(&database_name)
        .await
        .expect("Failed to drop test database");
}

#[tokio::test]
async fn user_can_register_and_login_and_get_me() {
    // Prepare
    let database_name = format!("hound_test_{}", Uuid::new_v4().simple());

    let db_pool = Database
        .clone(&database_name)
        .await
        .expect("Failed to clone test database");

    let address = hound::app::run(None, Some(db_pool))
        .expect("Failed to spawn app")
        .to_string();

    let client = reqwest::Client::new();
    let username = format!("user_{}", Uuid::new_v4().simple());
    let password = "password";

    // Act
    let register_response = client
        .post(&format!("http://127.0.0.1:{}/register", &address))
        .json(&serde_json::json!({
            "username": username,
            "password": password,
            "confirm_password": password
        }))
        .send()
        .await
        .expect("Failed to execute register request.");

    let login_response = client
        .post(&format!("http://127.0.0.1:{}/login", &address))
        .json(&serde_json::json!({
            "username": username,
            "password": password
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
    assert_eq!(201, register_response.status().as_u16());
    assert_eq!(200, login_response.status().as_u16());
    assert_eq!(200, me_response.status().as_u16());

    let register_body: serde_json::Value = register_response
        .json()
        .await
        .expect("Failed to parse register response body.");
    assert_eq!(register_body["username"], username);
    assert_eq!(register_body["role"], "user_free");

    let me_body: serde_json::Value = me_response
        .json()
        .await
        .expect("Failed to parse me response body.");
    assert_eq!(me_body["username"], username);
    assert_eq!(me_body["role"], "user_free");

    Database
        .drop(&database_name)
        .await
        .expect("Failed to drop test database");
}
