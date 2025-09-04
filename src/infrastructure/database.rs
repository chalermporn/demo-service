use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::{
    application::ports::{ProductRepository, RepositoryError},
    domain::{Price, Product, ProductId},
};

pub struct SqliteProductRepository {
    pool: SqlitePool,
}

fn parse_timestamp(timestamp_str: &str) -> Result<DateTime<Utc>, RepositoryError> {
    // Try ISO format first (with timezone)
    if let Ok(dt) = DateTime::parse_from_rfc3339(timestamp_str) {
        return Ok(dt.with_timezone(&Utc));
    }
    
    // Try SQLite format (YYYY-MM-DD HH:MM:SS)
    if let Ok(naive_dt) = chrono::NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S") {
        return Ok(naive_dt.and_utc());
    }
    
    Err(RepositoryError::DatabaseError(format!(
        "Unable to parse timestamp: {}",
        timestamp_str
    )))
}

impl SqliteProductRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn row_to_product(row: &sqlx::sqlite::SqliteRow) -> Result<Product, RepositoryError> {
        let id_str: String = row.get("id");
        let id = Uuid::parse_str(&id_str)
            .map_err(|e| RepositoryError::DatabaseError(format!("Invalid UUID: {}", e)))?;

        let price_value: f64 = row.get("price");
        let price = Price::new(price_value)
            .map_err(|e| RepositoryError::InvalidInput(e))?;

        let created_at_str: String = row.get("created_at");
        let updated_at_str: String = row.get("updated_at");

        let created_at = parse_timestamp(&created_at_str)?;
        let updated_at = parse_timestamp(&updated_at_str)?;

        Ok(Product {
            id: ProductId::from_uuid(id),
            name: row.get("name"),
            description: row.get("description"),
            price,
            category: row.get("category"),
            created_at,
            updated_at,
        })
    }
}

#[async_trait]
impl ProductRepository for SqliteProductRepository {
    async fn find_all(&self) -> Result<Vec<Product>, RepositoryError> {
        let rows = sqlx::query(
            "SELECT id, name, description, price, category, created_at, updated_at FROM products",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut products = Vec::new();
        for row in rows {
            products.push(Self::row_to_product(&row)?);
        }

        Ok(products)
    }

    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, RepositoryError> {
        let row = sqlx::query(
            "SELECT id, name, description, price, category, created_at, updated_at FROM products WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(Self::row_to_product(&row)?)),
            None => Ok(None),
        }
    }

    async fn save(&self, product: &Product) -> Result<(), RepositoryError> {
        sqlx::query(
            "INSERT INTO products (id, name, description, price, category, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(product.id.to_string())
        .bind(&product.name)
        .bind(&product.description)
        .bind(product.price.value())
        .bind(&product.category)
        .bind(product.created_at.format("%Y-%m-%d %H:%M:%S").to_string())
        .bind(product.updated_at.format("%Y-%m-%d %H:%M:%S").to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn update(&self, product: &Product) -> Result<(), RepositoryError> {
        sqlx::query(
            "UPDATE products SET name = ?, description = ?, price = ?, category = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&product.name)
        .bind(&product.description)
        .bind(product.price.value())
        .bind(&product.category)
        .bind(product.updated_at.format("%Y-%m-%d %H:%M:%S").to_string())
        .bind(product.id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: &ProductId) -> Result<bool, RepositoryError> {
        let result = sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }
}