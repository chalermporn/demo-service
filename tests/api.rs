use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use demo_service::build_app;
use serde_json::json;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use tower::util::ServiceExt;
use uuid::Uuid;

async fn test_pool() -> SqlitePool {
    let options = SqliteConnectOptions::new()
        .filename(":memory:")
        .create_if_missing(true);
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap()
}

async fn ensure_schema(pool: &SqlitePool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            price REAL NOT NULL,
            category TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(pool)
    .await
    .unwrap();
}

#[tokio::test]
async fn openapi_json_ok() {
    let pool = test_pool().await;
    ensure_schema(&pool).await;
    let app = build_app(pool).await;

    let res = app
        .oneshot(
            Request::builder()
                .uri("/api-docs/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn swagger_ui_renders() {
    let pool = test_pool().await;
    ensure_schema(&pool).await;
    let app = build_app(pool).await;

    let res = app
        .oneshot(
            Request::builder()
                .uri("/swagger-ui")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn list_empty() {
    let pool = test_pool().await;
    ensure_schema(&pool).await;
    let app = build_app(pool).await;

    let res = app
        .oneshot(
            Request::builder()
                .uri("/products")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(v.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn create_get_update_delete_flow() {
    let pool = test_pool().await;
    ensure_schema(&pool).await;
    let app = build_app(pool.clone()).await;

    // create
    let create_body = json!({
        "name": "Item",
        "description": "Desc",
        "price": 9.5,
        "category": "Cat"
    })
    .to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/products")
                .header("content-type", "application/json")
                .body(Body::from(create_body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let created: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let id = Uuid::parse_str(created["id"].as_str().unwrap()).unwrap();

    // get
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/products/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // update
    let up_body = json!({"name": "Item2", "price": 11.0, "category": "Cat2"}).to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/products/{id}"))
                .header("content-type", "application/json")
                .body(Body::from(up_body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let updated: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(updated["name"], "Item2");
    assert_eq!(updated["price"], 11.0);
    assert_eq!(updated["category"], "Cat2");

    // delete
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/products/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    // get 404
    let res = app
        .oneshot(
            Request::builder()
                .uri(format!("/products/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
