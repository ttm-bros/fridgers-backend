use actix_web::test;
use serde_json::{Value, json};

use super::helper;

#[actix_rt::test]
async fn test_login_success() {
    let (app, pool) = helper::create_test_app().await;

    // まずユーザーを登録
    let req = test::TestRequest::post()
        .uri("/v1/users")
        .set_json(json!({
            "name": "ログインテスト",
            "email": "login@example.com",
            "password": "password123"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    // ログイン
    let req = test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(json!({
            "email": "login@example.com",
            "password": "password123"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // ステータスコード200を確認
    assert_eq!(resp.status().as_u16(), 200);

    // レスポンスボディを確認
    let body: Value = test::read_body_json(resp).await;
    assert!(body["access_token"].is_string());
    assert_eq!(body["token_type"], "Bearer");

    // トークンが空でないことを確認
    let token = body["access_token"].as_str().unwrap();
    assert!(!token.is_empty());

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_login_wrong_email_returns_401() {
    let (app, pool) = helper::create_test_app().await;

    // 存在しないメールアドレスでログイン
    let req = test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(json!({
            "email": "nonexistent@example.com",
            "password": "password123"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // ステータスコード401を確認
    assert_eq!(resp.status().as_u16(), 401);

    let body: Value = test::read_body_json(resp).await;
    assert!(body["error"].is_string());

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_login_wrong_password_returns_401() {
    let (app, pool) = helper::create_test_app().await;

    // まずユーザーを登録
    let req = test::TestRequest::post()
        .uri("/v1/users")
        .set_json(json!({
            "name": "パスワードテスト",
            "email": "wrongpass@example.com",
            "password": "correct_password"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    // 間違ったパスワードでログイン
    let req = test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(json!({
            "email": "wrongpass@example.com",
            "password": "wrong_password"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // ステータスコード401を確認
    assert_eq!(resp.status().as_u16(), 401);

    let body: Value = test::read_body_json(resp).await;
    assert!(body["error"].is_string());

    helper::cleanup_users(&pool).await;
}
