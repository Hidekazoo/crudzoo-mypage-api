use crate::helpers::spawn_app;
use actix_web::error::UrlencodedError::ContentType;
use reqwest::header;
use reqwest::header::{AUTHORIZATION, WWW_AUTHENTICATE};

#[tokio::test]
async fn requests_missing_authorization_are_rejected() {
    let app = spawn_app().await;
    let response = reqwest::Client::new()
        .get(&format!("{}/post", &app.address))
        .header("content-type", "application/json")
        .header(AUTHORIZATION, "Bar!!")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
}
