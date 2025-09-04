use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::value_objects::{Price, ProductId};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub description: String,
    pub price: Price,
    pub category: String,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,
}

impl Product {
    pub fn new(
        name: String,
        description: String,
        price: Price,
        category: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: ProductId::new(),
            name,
            description,
            price,
            category,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    pub fn update_description(&mut self, description: String) {
        self.description = description;
        self.updated_at = Utc::now();
    }

    pub fn update_price(&mut self, price: Price) {
        self.price = price;
        self.updated_at = Utc::now();
    }

    pub fn update_category(&mut self, category: String) {
        self.category = category;
        self.updated_at = Utc::now();
    }
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