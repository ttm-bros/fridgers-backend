use actix_web::test;
use serde_json::{json, Value};

use super::helper;

#[actix_rt::test]
async fn test_register_user_success() {
    let (app, pool) = helper::create_test_app().await;

    let req = test::TestRequest::post()
        .uri("/v1/users")
        .set_json(json!({
            "name": "テストユーザー",
            "email": "test@example.com",
            "password": "password123"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // ステータスコード201を確認
    assert_eq!(resp.status().as_u16(), 201);

    // レスポンスボディを確認
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["name"], "テストユーザー");
    assert!(body["id"].is_string());

    // UUIDとして有効か確認
    let id = body["id"].as_str().unwrap();
    uuid::Uuid::parse_str(id).expect("Response id should be a valid UUID");

    // DBにレコードが保存されているか確認
    let row = sqlx::query_as::<_, (String,)>("SELECT name FROM users WHERE id = $1::uuid")
        .bind(id)
        .fetch_one(&pool)
        .await
        .expect("User should exist in database");
    assert_eq!(row.0, "テストユーザー");

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_register_user_empty_name_returns_400() {
    let (app, pool) = helper::create_test_app().await;

    let req = test::TestRequest::post()
        .uri("/v1/users")
        .set_json(json!({
            "name": "",
            "email": "test@example.com",
            "password": "password123"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // ステータスコード400を確認
    assert_eq!(resp.status().as_u16(), 400);

    // エラーレスポンスのボディを確認
    let body: Value = test::read_body_json(resp).await;
    assert!(body["error"].is_string());

    // DBにレコードが保存されていないことを確認
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .expect("Failed to count users");
    assert_eq!(count.0, 0);

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_register_user_duplicate_email_returns_409() {
    let (app, pool) = helper::create_test_app().await;

    // 1人目のユーザー登録
    let req = test::TestRequest::post()
        .uri("/v1/users")
        .set_json(json!({
            "name": "ユーザー1",
            "email": "duplicate@example.com",
            "password": "password123"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    // 同じメールアドレスで2人目を登録
    let req = test::TestRequest::post()
        .uri("/v1/users")
        .set_json(json!({
            "name": "ユーザー2",
            "email": "duplicate@example.com",
            "password": "password456"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // 409 Conflictを確認
    assert_eq!(resp.status().as_u16(), 409);

    helper::cleanup_users(&pool).await;
}
