use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, Json},
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

pub type DatabasePool = SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Product {
    #[schema(value_type = String, format = "uuid")]
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub category: Option<String>,
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
        schemas(Product, CreateProductRequest, UpdateProductRequest)
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

pub fn build_app(pool: DatabasePool) -> Router {
    Router::new()
        .route("/products", get(get_all_products))
        .route("/products", post(create_product))
        .route("/products/:id", get(get_product))
        .route("/products/:id", put(update_product))
        .route("/products/:id", delete(delete_product))
        .route("/api-docs/openapi.json", get(openapi_json))
        .route("/swagger-ui", get(swagger_ui))
        .with_state(pool)
}

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
    axum::extract::State(pool): axum::extract::State<DatabasePool>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT id, name, description, price, category, created_at, updated_at FROM products",
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let products: Vec<Product> = rows
        .into_iter()
        .map(|row| Product {
            id: Uuid::parse_str(&row.get::<String, _>("id")).unwrap(),
            name: row.get("name"),
            description: row.get("description"),
            price: row.get("price"),
            category: row.get("category"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(Json(products))
}

#[utoipa::path(
    post,
    path = "/products",
    request_body = CreateProductRequest,
    responses(
        (status = 201, description = "Product created successfully", body = Product),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
async fn create_product(
    axum::extract::State(pool): axum::extract::State<DatabasePool>,
    Json(request): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO products (id, name, description, price, category, created_at, updated_at) VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
    )
    .bind(id.to_string())
    .bind(&request.name)
    .bind(&request.description)
    .bind(request.price)
    .bind(&request.category)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get the created product with actual timestamps
    let row = sqlx::query(
        "SELECT id, name, description, price, category, created_at, updated_at FROM products WHERE id = ?",
    )
    .bind(id.to_string())
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let product = Product {
        id,
        name: request.name,
        description: request.description,
        price: request.price,
        category: request.category,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };

    Ok((StatusCode::CREATED, Json(product)))
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
    axum::extract::State(pool): axum::extract::State<DatabasePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Product>, StatusCode> {
    let row = sqlx::query(
        "SELECT id, name, description, price, category, created_at, updated_at FROM products WHERE id = ?",
    )
    .bind(id.to_string())
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(row) => {
            let product = Product {
                id: Uuid::parse_str(&row.get::<String, _>("id")).unwrap(),
                name: row.get("name"),
                description: row.get("description"),
                price: row.get("price"),
                category: row.get("category"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(Json(product))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[utoipa::path(
    put,
    path = "/products/{id}",
    params(("id" = Uuid, Path, description = "Product ID")),
    request_body = UpdateProductRequest,
    responses(
        (status = 200, description = "Product updated successfully", body = Product),
        (status = 404, description = "Product not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "products"
)]
async fn update_product(
    axum::extract::State(pool): axum::extract::State<DatabasePool>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateProductRequest>,
) -> Result<Json<Product>, StatusCode> {
    // First check if product exists
    let existing_row = sqlx::query(
        "SELECT id, name, description, price, category, created_at, updated_at FROM products WHERE id = ?",
    )
    .bind(id.to_string())
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut product = match existing_row {
        Some(row) => Product {
            id: Uuid::parse_str(&row.get::<String, _>("id")).unwrap(),
            name: row.get("name"),
            description: row.get("description"),
            price: row.get("price"),
            category: row.get("category"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        },
        None => return Err(StatusCode::NOT_FOUND),
    };

    // Update fields if provided
    if let Some(name) = request.name {
        product.name = name;
    }
    if let Some(description) = request.description {
        product.description = description;
    }
    if let Some(price) = request.price {
        product.price = price;
    }
    if let Some(category) = request.category {
        product.category = category;
    }
    // Update in database
    sqlx::query(
        "UPDATE products SET name = ?, description = ?, price = ?, category = ?, updated_at = datetime('now') WHERE id = ?",
    )
    .bind(&product.name)
    .bind(&product.description)
    .bind(product.price)
    .bind(&product.category)
    .bind(id.to_string())
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get updated timestamp
    let updated_row = sqlx::query("SELECT updated_at FROM products WHERE id = ?")
        .bind(id.to_string())
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    product.updated_at = updated_row.get("updated_at");

    Ok(Json(product))
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
    axum::extract::State(pool): axum::extract::State<DatabasePool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(id.to_string())
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
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
