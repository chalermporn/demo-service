use std::sync::Arc;

use crate::domain::{CreateProductRequest, Price, Product, ProductId, UpdateProductRequest};

use super::ports::{ProductRepository, RepositoryError};

pub struct ProductService {
    repository: Arc<dyn ProductRepository>,
}

impl ProductService {
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_products(&self) -> Result<Vec<Product>, RepositoryError> {
        self.repository.find_all().await
    }

    pub async fn get_product_by_id(&self, id: &ProductId) -> Result<Option<Product>, RepositoryError> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_product(&self, request: CreateProductRequest) -> Result<Product, RepositoryError> {
        let price = Price::new(request.price)
            .map_err(|e| RepositoryError::InvalidInput(e))?;

        let product = Product::new(
            request.name,
            request.description,
            price,
            request.category,
        );

        self.repository.save(&product).await?;
        Ok(product)
    }

    pub async fn update_product(
        &self,
        id: &ProductId,
        request: UpdateProductRequest,
    ) -> Result<Option<Product>, RepositoryError> {
        let mut product = match self.repository.find_by_id(id).await? {
            Some(product) => product,
            None => return Ok(None),
        };

        if let Some(name) = request.name {
            product.update_name(name);
        }

        if let Some(description) = request.description {
            product.update_description(description);
        }

        if let Some(price_value) = request.price {
            let price = Price::new(price_value)
                .map_err(|e| RepositoryError::InvalidInput(e))?;
            product.update_price(price);
        }

        if let Some(category) = request.category {
            product.update_category(category);
        }

        self.repository.update(&product).await?;
        Ok(Some(product))
    }

    pub async fn delete_product(&self, id: &ProductId) -> Result<bool, RepositoryError> {
        self.repository.delete(id).await
    }
}