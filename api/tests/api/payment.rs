use crate::helpers::spawn_app;
use reqwest::header::AUTHORIZATION;
use std::collections::HashMap;

#[tokio::test]
async fn get_payment_type_requests_missing_authorization_are_rejected() {
    let app = spawn_app().await;
    let response = reqwest::Client::new()
        .get(&format!("{}/payment_type", &app.address))
        .header("content-type", "application/json")
        .header(AUTHORIZATION, "Bar!!")
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn find_payment_requests_missing_authorization_are_rejected() {
    let app = spawn_app().await;
    let response = reqwest::Client::new()
        .get(&format!("{}/user/1/payment", &app.address))
        .header("content-type", "application/json")
        .header(AUTHORIZATION, "Bar!!")
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn create_payment_requests_missing_authorization_are_rejected() {
    let app = spawn_app().await;
    let mut map = HashMap::new();
    map.insert("payment_type_id", 1);
    map.insert("user_id", 1);
    map.insert("amount", 1000);

    let response = reqwest::Client::new()
        .post(&format!("{}/payment", &app.address))
        .header("content-type", "application/json")
        .header(AUTHORIZATION, "Bar!!")
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(401, response.status().as_u16());
}
