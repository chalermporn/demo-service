use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use tokio::net::TcpListener;
use uuid::Uuid;

type DatabasePool = SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub category: Option<String>,
}

#[tokio::main]
async fn main() {
    // Initialize database
    let database_url = "sqlite:./products.db";
    
    // Create database file if it doesn't exist
    if !std::path::Path::new("./products.db").exists() {
        std::fs::File::create("./products.db").unwrap();
    }
    
    let pool = SqlitePool::connect(database_url).await.unwrap();
    
    // Create table if it doesn't exist
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            price REAL NOT NULL,
            category TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await
    .unwrap();

    let app = Router::new()
        .route("/products", get(get_all_products))
        .route("/products", post(create_product))
        .route("/products/:id", get(get_product))
        .route("/products/:id", put(update_product))
        .route("/products/:id", delete(delete_product))
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Product service running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn get_all_products(
    axum::extract::State(pool): axum::extract::State<DatabasePool>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT id, name, description, price, category, created_at, updated_at FROM products"
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

async fn create_product(
    axum::extract::State(pool): axum::extract::State<DatabasePool>,
    Json(request): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    let id = Uuid::new_v4();
    let now = "CURRENT_TIMESTAMP";

    sqlx::query(
        "INSERT INTO products (id, name, description, price, category, created_at, updated_at) VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
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
        "SELECT id, name, description, price, category, created_at, updated_at FROM products WHERE id = ?"
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

async fn get_product(
    axum::extract::State(pool): axum::extract::State<DatabasePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Product>, StatusCode> {
    let row = sqlx::query(
        "SELECT id, name, description, price, category, created_at, updated_at FROM products WHERE id = ?"
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

async fn update_product(
    axum::extract::State(pool): axum::extract::State<DatabasePool>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateProductRequest>,
) -> Result<Json<Product>, StatusCode> {
    // First check if product exists
    let existing_row = sqlx::query(
        "SELECT id, name, description, price, category, created_at, updated_at FROM products WHERE id = ?"
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
        "UPDATE products SET name = ?, description = ?, price = ?, category = ?, updated_at = datetime('now') WHERE id = ?"
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
    let updated_row = sqlx::query(
        "SELECT updated_at FROM products WHERE id = ?"
    )
    .bind(id.to_string())
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    product.updated_at = updated_row.get("updated_at");

    Ok(Json(product))
}

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