use demo_service::build_app;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Initialize database
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./products.db".to_string());
    
    // Extract file path from database URL for SQLite
    let db_file_path = if database_url.starts_with("sqlite:") {
        &database_url[7..] // Remove "sqlite:" prefix
    } else {
        "./products.db"
    };

    // Create database file if it doesn't exist
    if !std::path::Path::new(db_file_path).exists() {
        if let Some(parent) = std::path::Path::new(db_file_path).parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::File::create(db_file_path).unwrap();
    }

    let pool = SqlitePool::connect(&database_url).await.unwrap();

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
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    let app = build_app(pool).await;

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Product service running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

// application logic moved to library (see src/lib.rs)
