use async_trait::async_trait;
use std::result::Result;

use crate::domain::{Product, ProductId};

#[derive(Debug, Clone)]
pub enum RepositoryError {
    NotFound,
    DatabaseError(String),
    InvalidInput(String),
}

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Product>, RepositoryError>;
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, RepositoryError>;
    async fn save(&self, product: &Product) -> Result<(), RepositoryError>;
    async fn update(&self, product: &Product) -> Result<(), RepositoryError>;
    async fn delete(&self, id: &ProductId) -> Result<bool, RepositoryError>;
}