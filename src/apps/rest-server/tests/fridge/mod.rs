use actix_web::test;
use serde_json::{Value, json};

use super::helper;

/// テスト用ユーザーを登録してログインし、(user_id, jwt) を返す
async fn register_and_login<S>(app: &S, name: &str, email: &str, password: &str) -> (String, String)
where
    S: actix_web::dev::Service<
            actix_http::Request,
            Response = actix_web::dev::ServiceResponse,
            Error = actix_web::Error,
        >,
{
    let req = test::TestRequest::post()
        .uri("/v1/users")
        .set_json(json!({
            "name": name,
            "email": email,
            "password": password,
        }))
        .to_request();
    let resp = test::call_service(app, req).await;
    assert_eq!(resp.status().as_u16(), 201);
    let body: Value = test::read_body_json(resp).await;
    let user_id = body["id"].as_str().unwrap().to_string();

    let req = test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(json!({
            "email": email,
            "password": password,
        }))
        .to_request();
    let resp = test::call_service(app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    let body: Value = test::read_body_json(resp).await;
    let token = body["access_token"].as_str().unwrap().to_string();

    (user_id, token)
}

/// 指定ユーザーが所有する冷蔵庫を作成する
async fn create_fridge<S>(app: &S, owner_user_id: &str, name: &str) -> String
where
    S: actix_web::dev::Service<
            actix_http::Request,
            Response = actix_web::dev::ServiceResponse,
            Error = actix_web::Error,
        >,
{
    let req = test::TestRequest::post()
        .uri("/v1/fridges")
        .set_json(json!({
            "name": name,
            "owner_user_id": owner_user_id,
        }))
        .to_request();
    let resp = test::call_service(app, req).await;
    assert_eq!(resp.status().as_u16(), 201);
    let body: Value = test::read_body_json(resp).await;
    body["id"].as_str().unwrap().to_string()
}

#[actix_rt::test]
async fn test_list_fridges_success_returns_owned_fridges() {
    let (app, pool) = helper::create_test_app().await;

    let (user_id, token) =
        register_and_login(&app, "オーナーA", "owner-a@example.com", "password123").await;

    let id1 = create_fridge(&app, &user_id, "主冷蔵庫").await;
    let id2 = create_fridge(&app, &user_id, "サブ冷蔵庫").await;

    let req = test::TestRequest::get()
        .uri("/v1/fridges")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: Value = test::read_body_json(resp).await;
    let fridges = body["fridges"].as_array().expect("fridges should be array");
    assert_eq!(fridges.len(), 2);

    let returned_ids: Vec<&str> = fridges.iter().map(|f| f["id"].as_str().unwrap()).collect();
    assert!(returned_ids.contains(&id1.as_str()));
    assert!(returned_ids.contains(&id2.as_str()));
    for f in fridges {
        assert_eq!(f["owner_user_id"], user_id);
    }

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_list_fridges_returns_empty_when_user_has_none() {
    let (app, pool) = helper::create_test_app().await;

    let (_user_id, token) =
        register_and_login(&app, "空ユーザー", "empty@example.com", "password123").await;

    let req = test::TestRequest::get()
        .uri("/v1/fridges")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: Value = test::read_body_json(resp).await;
    let fridges = body["fridges"].as_array().expect("fridges should be array");
    assert!(fridges.is_empty());

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_list_fridges_isolates_other_users_fridges() {
    let (app, pool) = helper::create_test_app().await;

    // ユーザーA: 冷蔵庫を2件持つ
    let (user_a_id, _token_a) =
        register_and_login(&app, "ユーザーA", "user-a@example.com", "password123").await;
    create_fridge(&app, &user_a_id, "Aの冷蔵庫1").await;
    create_fridge(&app, &user_a_id, "Aの冷蔵庫2").await;

    // ユーザーB: 冷蔵庫を1件持つ
    let (user_b_id, token_b) =
        register_and_login(&app, "ユーザーB", "user-b@example.com", "password123").await;
    let b_fridge_id = create_fridge(&app, &user_b_id, "Bの冷蔵庫").await;

    // BがGETすると自分の1件だけが返る
    let req = test::TestRequest::get()
        .uri("/v1/fridges")
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: Value = test::read_body_json(resp).await;
    let fridges = body["fridges"].as_array().unwrap();
    assert_eq!(fridges.len(), 1);
    assert_eq!(fridges[0]["id"], b_fridge_id);
    assert_eq!(fridges[0]["owner_user_id"], user_b_id);

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_list_fridges_without_auth_header_returns_401() {
    let (app, pool) = helper::create_test_app().await;

    let req = test::TestRequest::get().uri("/v1/fridges").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);

    let body: Value = test::read_body_json(resp).await;
    assert!(body["error"].is_string());

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_list_fridges_with_invalid_token_returns_401() {
    let (app, pool) = helper::create_test_app().await;

    let req = test::TestRequest::get()
        .uri("/v1/fridges")
        .insert_header(("Authorization", "Bearer this-is-not-a-valid-jwt"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);

    let body: Value = test::read_body_json(resp).await;
    assert!(body["error"].is_string());

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_list_fridges_with_non_bearer_scheme_returns_401() {
    let (app, pool) = helper::create_test_app().await;

    let req = test::TestRequest::get()
        .uri("/v1/fridges")
        .insert_header(("Authorization", "Basic dXNlcjpwYXNz"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_list_fridges_accepts_lowercase_bearer_scheme() {
    let (app, pool) = helper::create_test_app().await;

    let (_user_id, token) =
        register_and_login(&app, "ケース", "case@example.com", "password123").await;

    let req = test::TestRequest::get()
        .uri("/v1/fridges")
        .insert_header(("Authorization", format!("bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    // RFC 7235 ではスキーム名は大文字小文字を区別しないため、200を期待
    assert_eq!(resp.status().as_u16(), 200);

    helper::cleanup_users(&pool).await;
}
