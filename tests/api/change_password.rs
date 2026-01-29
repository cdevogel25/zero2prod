use crate::helpers::{assert_is_redirect_to, spawn_app};
use uuid::Uuid;
use rand::{Rng, distr::Alphanumeric};

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    // arrange
    let app = spawn_app().await;

    // act
    let response = app.get_change_password().await;

    // assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    // arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // act
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    // arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    // act 1: login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    }))
    .await;

    // act 2: try to change the password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &another_new_password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/password");

    // act 3: follow the redirect
    let html_page = app.get_change_password_html().await;
    // dbg!(&html_page);
    assert!(html_page.contains(
        "<p><i>You entered two different new passwords - the field values must match.</i></p>"
    ));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    // arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    // act 1: login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    }))
    .await;

    //act 2: try to change the password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    
    // assert
    assert_is_redirect_to(&response, "/admin/password");

    // act 3: follow the redirect
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains(
        "<p><i>The current password is incorrect.</i></p>"
    ));
}

#[tokio::test]
async fn new_password_is_not_too_short() {
    // arrange
    let app = spawn_app().await;
    let new_password = "EpPR5ZaxX4fh".to_string();

    // act 1: login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    // act 2: try to change the password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password
        }))
        .await;

    // assert
    assert_is_redirect_to(&response, "/admin/password");

    // act 3: follow the redirect
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains(
        "<p><i>Your new password is too short - it must be at least 13 characters.</i></p>"
    ));
}

#[tokio::test]
async fn new_password_is_not_too_long() {
    // arrange
    let app = spawn_app().await;
    let new_password: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(129)
        .map(char::from)
        .collect();

    // act 1: login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    // act 2: try to change the password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;

    // assert
    assert_is_redirect_to(&response, "/admin/password");

    // act 3: follow the redirect
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains(
        "<p><i>Your new password is too long - it must be shorter than 129 characters.</i></p>"
    ));
}