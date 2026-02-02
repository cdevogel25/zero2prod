use crate::helpers::spawn_app;
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    // arrange
    let app = spawn_app().await;

    // act
    let response = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    // assert
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    // arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);
    // act
    let response = reqwest::get(confirmation_links.html).await.unwrap();

    // assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn clicking_on_the_confirmation_link_confirms_a_subscriber() {
    // arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    // act
    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    // assert
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "confirmed");
}

#[tokio::test]
async fn clicking_on_the_confirmation_link_twice_returns_a_conflict_error() {
    // arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    // act 1: click the confirmation link once
    reqwest::get(confirmation_links.html.clone())
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    // assert 1: subscriber is confirmed
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.status, "confirmed");

    // act 2: click the link a second time
    let response = reqwest::get(confirmation_links.html)
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 409);
}

#[tokio::test]
async fn subscribing_twice_without_confirming_returns_identical_confirmation_links() {
    // arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // act
    app.post_subscriptions(body.into()).await;
    let first_email_request = &app.email_server.received_requests().await.unwrap()[0];
    
    app.post_subscriptions(body.into()).await;
    let second_email_request = &app.email_server.received_requests().await.unwrap()[0];
    
    let first_confirmation_links = app.get_confirmation_links(&first_email_request);
    let second_confirmation_links = app.get_confirmation_links(&second_email_request);

    // assert
    assert_eq!(first_confirmation_links.html, second_confirmation_links.html)
}

// #[tokio::test]
async fn _attempting_to_subscribe_after_clicking_the_confirmation_link() {
    // arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // act
    app.post_subscriptions(body.into()).await;
    let first_email_request = &app.email_server.received_requests().await.unwrap()[0];
    let first_confirmation_links = app.get_confirmation_links(&first_email_request);

    reqwest::get(first_confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    app.post_subscriptions(body.into()).await;
    let second_email_request = &app.email_server.received_requests().await.unwrap()[0];
    let second_confirmation_link = app.get_confirmation_links(&second_email_request);

    dbg!(second_confirmation_link);
}