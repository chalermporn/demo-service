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

impl CreateProductRequest {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate name
        if self.name.trim().is_empty() {
            errors.push("Name cannot be empty".to_string());
        }
        if self.name.len() > 100 {
            errors.push("Name cannot exceed 100 characters".to_string());
        }

        // Validate description
        if self.description.trim().is_empty() {
            errors.push("Description cannot be empty".to_string());
        }
        if self.description.len() > 500 {
            errors.push("Description cannot exceed 500 characters".to_string());
        }

        // Validate price
        if self.price < 0.0 {
            errors.push("Price cannot be negative".to_string());
        }
        if self.price > 999999.99 {
            errors.push("Price cannot exceed 999,999.99".to_string());
        }

        // Validate category
        if self.category.trim().is_empty() {
            errors.push("Category cannot be empty".to_string());
        }
        if self.category.len() > 50 {
            errors.push("Category cannot exceed 50 characters".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub category: Option<String>,
}

impl UpdateProductRequest {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate name if present
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                errors.push("Name cannot be empty".to_string());
            }
            if name.len() > 100 {
                errors.push("Name cannot exceed 100 characters".to_string());
            }
        }

        // Validate description if present
        if let Some(ref description) = self.description {
            if description.trim().is_empty() {
                errors.push("Description cannot be empty".to_string());
            }
            if description.len() > 500 {
                errors.push("Description cannot exceed 500 characters".to_string());
            }
        }

        // Validate price if present
        if let Some(price) = self.price {
            if price < 0.0 {
                errors.push("Price cannot be negative".to_string());
            }
            if price > 999999.99 {
                errors.push("Price cannot exceed 999,999.99".to_string());
            }
        }

        // Validate category if present
        if let Some(ref category) = self.category {
            if category.trim().is_empty() {
                errors.push("Category cannot be empty".to_string());
            }
            if category.len() > 50 {
                errors.push("Category cannot exceed 50 characters".to_string());
            }
        }

        // At least one field must be provided for update
        if self.name.is_none()
            && self.description.is_none()
            && self.price.is_none()
            && self.category.is_none()
        {
            errors.push("At least one field must be provided for update".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}