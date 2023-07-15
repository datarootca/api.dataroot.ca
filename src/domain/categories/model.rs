use chrono::{DateTime, Utc};

#[cfg(test)]
use crate::api::utils::random_number;

#[derive(Debug, Clone)]
pub struct CategoryCreateModel {
    pub name: String,
    pub description: Option<String>,
}
impl CategoryCreateModel {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            name,
            description,
        }
    }
}

#[cfg(test)]
impl CategoryCreateModel {
    pub fn mock_default() -> Self {
        Self {
            name: "Burgers".to_string(),
            description: Some("The Big Burgers".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CategoryUpdateModel {
    pub name: String,
    pub description: Option<String>,
}
impl CategoryUpdateModel {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            name,
            description,
        }
    }
}
#[cfg(test)]
impl CategoryUpdateModel {
    pub fn mock_default() -> Self {
        Self {
            name: "French fries".to_string(),
            description: Some("The French fries".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CategoryModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[cfg(test)]
impl CategoryModel {
    pub fn mock_default() -> Self {
        Self {
            id: random_number(),
            name: "Burgers".to_string(),
            description: Some("The Big Burgers".to_string()),
            is_active: true,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}
