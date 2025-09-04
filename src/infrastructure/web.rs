use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Json},
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    application::{ProductService, RepositoryError},
    domain::{CreateProductRequest, Price, Product, ProductId, UpdateProductRequest},
};

pub struct WebAdapter {
    service: Arc<ProductService>,
}

impl WebAdapter {
    pub fn new(service: Arc<ProductService>) -> Self {
        Self { service }
    }

    pub fn router(&self) -> Router {
        Router::new()
            .route("/products", get(get_all_products))
            .route("/products", post(create_product))
            .route("/products/:id", get(get_product))
            .route("/products/:id", put(update_product))
            .route("/products/:id", delete(delete_product))
            .route("/api-docs/openapi.json", get(openapi_json))
            .route("/swagger-ui", get(swagger_ui))
            .with_state(self.service.clone())
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_all_products,
        create_product,
        get_product,
        update_product,
        delete_product
    ),
    components(
        schemas(Product, CreateProductRequest, UpdateProductRequest, ProductId, Price)
    ),
    tags(
        (name = "products", description = "Product management API")
    ),
    info(
        title = "Demo Service API",
        description = "A simple product management API",
        version = "0.1.0"
    )
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/products",
    responses(
        (status = 200, description = "List all products successfully", body = [Product]),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
async fn get_all_products(
    State(service): State<Arc<ProductService>>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    match service.get_all_products().await {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/products",
    request_body = CreateProductRequest,
    responses(
        (status = 201, description = "Product created successfully", body = Product),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
async fn create_product(
    State(service): State<Arc<ProductService>>,
    Json(request): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    match service.create_product(request).await {
        Ok(product) => Ok((StatusCode::CREATED, Json(product))),
        Err(RepositoryError::InvalidInput(_)) => Err(StatusCode::BAD_REQUEST),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    get,
    path = "/products/{id}",
    params(("id" = Uuid, Path, description = "Product ID")),
    responses(
        (status = 200, description = "Product found successfully", body = Product),
        (status = 404, description = "Product not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
async fn get_product(
    State(service): State<Arc<ProductService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Product>, StatusCode> {
    let product_id = ProductId::from_uuid(id);
    
    match service.get_product_by_id(&product_id).await {
        Ok(Some(product)) => Ok(Json(product)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    put,
    path = "/products/{id}",
    params(("id" = Uuid, Path, description = "Product ID")),
    request_body = UpdateProductRequest,
    responses(
        (status = 200, description = "Product updated successfully", body = Product),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Product not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
async fn update_product(
    State(service): State<Arc<ProductService>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateProductRequest>,
) -> Result<Json<Product>, StatusCode> {
    let product_id = ProductId::from_uuid(id);
    
    match service.update_product(&product_id, request).await {
        Ok(Some(product)) => Ok(Json(product)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(RepositoryError::InvalidInput(_)) => Err(StatusCode::BAD_REQUEST),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    delete,
    path = "/products/{id}",
    params(("id" = Uuid, Path, description = "Product ID")),
    responses(
        (status = 204, description = "Product deleted successfully"),
        (status = 404, description = "Product not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
async fn delete_product(
    State(service): State<Arc<ProductService>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let product_id = ProductId::from_uuid(id);
    
    match service.delete_product(&product_id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn openapi_json() -> Result<Json<utoipa::openapi::OpenApi>, StatusCode> {
    Ok(Json(ApiDoc::openapi()))
}

async fn swagger_ui() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Swagger UI</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css" />
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js"></script>
    <script>
        SwaggerUIBundle({
            url: '/api-docs/openapi.json',
            dom_id: '#swagger-ui',
            presets: [
                SwaggerUIBundle.presets.apis,
                SwaggerUIBundle.presets.standalone
            ]
        });
    </script>
</body>
</html>"#,
    )
}