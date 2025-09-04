use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[schema(value_type = String, format = "uuid")]
pub struct ProductId(pub Uuid);

impl ProductId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for ProductId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[schema(value_type = f64)]
pub struct Price(pub f64);

impl Price {
    pub fn new(value: f64) -> Result<Self, String> {
        if value < 0.0 {
            Err("Price cannot be negative".to_string())
        } else {
            Ok(Self(value))
        }
    }
    
    pub fn value(&self) -> f64 {
        self.0
    }
}