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

/// 認証済ユーザー（token のオーナー）として冷蔵庫を作成する
async fn create_fridge<S>(app: &S, token: &str, name: &str) -> String
where
    S: actix_web::dev::Service<
            actix_http::Request,
            Response = actix_web::dev::ServiceResponse,
            Error = actix_web::Error,
        >,
{
    let req = test::TestRequest::post()
        .uri("/v1/fridges")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({ "name": name }))
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

    let id1 = create_fridge(&app, &token, "主冷蔵庫").await;
    let id2 = create_fridge(&app, &token, "サブ冷蔵庫").await;

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
    let (_user_a_id, token_a) =
        register_and_login(&app, "ユーザーA", "user-a@example.com", "password123").await;
    create_fridge(&app, &token_a, "Aの冷蔵庫1").await;
    create_fridge(&app, &token_a, "Aの冷蔵庫2").await;

    // ユーザーB: 冷蔵庫を1件持つ
    let (user_b_id, token_b) =
        register_and_login(&app, "ユーザーB", "user-b@example.com", "password123").await;
    let b_fridge_id = create_fridge(&app, &token_b, "Bの冷蔵庫").await;

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

// ==== 認可テスト ====

#[actix_rt::test]
async fn test_create_fridge_without_auth_returns_401() {
    let (app, pool) = helper::create_test_app().await;

    let req = test::TestRequest::post()
        .uri("/v1/fridges")
        .set_json(json!({ "name": "noauth" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_create_fridge_uses_jwt_user_as_owner() {
    let (app, pool) = helper::create_test_app().await;

    let (user_id, token) =
        register_and_login(&app, "JWTオーナー", "jwt-owner@example.com", "password123").await;

    // body に owner_user_id を含めず作成
    let req = test::TestRequest::post()
        .uri("/v1/fridges")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({ "name": "JWT冷蔵庫" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["owner_user_id"], user_id);

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_get_fridge_of_other_user_returns_404() {
    let (app, pool) = helper::create_test_app().await;

    let (_user_a_id, token_a) =
        register_and_login(&app, "owner", "get-owner@example.com", "password123").await;
    let a_fridge_id = create_fridge(&app, &token_a, "Aの冷蔵庫").await;

    let (_user_b_id, token_b) =
        register_and_login(&app, "stranger", "get-stranger@example.com", "password123").await;

    let req = test::TestRequest::get()
        .uri(&format!("/v1/fridges/{}", a_fridge_id))
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    // 他人の冷蔵庫は存在を漏らさず NotFound
    assert_eq!(resp.status().as_u16(), 404);

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_get_fridge_owner_succeeds() {
    let (app, pool) = helper::create_test_app().await;

    let (_user_id, token) =
        register_and_login(&app, "owner", "get-self@example.com", "password123").await;
    let fridge_id = create_fridge(&app, &token, "自分の冷蔵庫").await;

    let req = test::TestRequest::get()
        .uri(&format!("/v1/fridges/{}", fridge_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_delete_fridge_of_other_user_returns_404_and_does_not_delete() {
    let (app, pool) = helper::create_test_app().await;

    let (_user_a_id, token_a) =
        register_and_login(&app, "owner", "del-owner@example.com", "password123").await;
    let a_fridge_id = create_fridge(&app, &token_a, "Aの冷蔵庫").await;

    let (_user_b_id, token_b) =
        register_and_login(&app, "stranger", "del-stranger@example.com", "password123").await;

    // B が A の冷蔵庫を消そうとする → 404
    let req = test::TestRequest::delete()
        .uri(&format!("/v1/fridges/{}", a_fridge_id))
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);

    // A 自身が GET したら依然存在することを確認
    let req = test::TestRequest::get()
        .uri(&format!("/v1/fridges/{}", a_fridge_id))
        .insert_header(("Authorization", format!("Bearer {}", token_a)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_create_compartment_in_other_user_fridge_returns_404() {
    let (app, pool) = helper::create_test_app().await;

    let (_user_a_id, token_a) =
        register_and_login(&app, "owner", "comp-owner@example.com", "password123").await;
    let a_fridge_id = create_fridge(&app, &token_a, "Aの冷蔵庫").await;

    let (_user_b_id, token_b) =
        register_and_login(&app, "stranger", "comp-stranger@example.com", "password123").await;

    let req = test::TestRequest::post()
        .uri(&format!("/v1/fridges/{}/compartments", a_fridge_id))
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .set_json(json!({ "name": "侵入" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);

    helper::cleanup_users(&pool).await;
}

#[actix_rt::test]
async fn test_create_item_in_other_user_compartment_returns_404() {
    let (app, pool) = helper::create_test_app().await;

    let (_user_a_id, token_a) =
        register_and_login(&app, "owner", "item-owner@example.com", "password123").await;
    let a_fridge_id = create_fridge(&app, &token_a, "Aの冷蔵庫").await;

    // A 自身でコンパートメント作成
    let req = test::TestRequest::post()
        .uri(&format!("/v1/fridges/{}/compartments", a_fridge_id))
        .insert_header(("Authorization", format!("Bearer {}", token_a)))
        .set_json(json!({ "name": "野菜室" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);
    let body: Value = test::read_body_json(resp).await;
    let compartment_id = body["id"].as_str().unwrap().to_string();

    let (_user_b_id, token_b) =
        register_and_login(&app, "stranger", "item-stranger@example.com", "password123").await;

    let req = test::TestRequest::post()
        .uri(&format!(
            "/v1/fridges/{}/compartments/{}/items",
            a_fridge_id, compartment_id
        ))
        .insert_header(("Authorization", format!("Bearer {}", token_b)))
        .set_json(json!({
            "name": "侵入物",
            "quantity": 1.0,
            "unit": "個",
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);

    helper::cleanup_users(&pool).await;
}
