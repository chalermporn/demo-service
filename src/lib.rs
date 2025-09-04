pub mod application;
pub mod domain;
pub mod infrastructure;

use std::sync::Arc;

use application::ProductService;
use infrastructure::{SqliteProductRepository, WebAdapter};
use sqlx::SqlitePool;

pub async fn build_app(pool: SqlitePool) -> axum::Router {
    let repository = Arc::new(SqliteProductRepository::new(pool));
    let service = Arc::new(ProductService::new(repository));
    let adapter = WebAdapter::new(service);
    
    adapter.router()
}
