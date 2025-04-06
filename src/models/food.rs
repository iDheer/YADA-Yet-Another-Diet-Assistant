
// src/models/food.rs
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum FoodType {
    Basic,
    Composite,
}

#[derive(Debug, Clone)]
pub struct Food {
    pub id: String,
    pub name: String,
    pub keywords: HashSet<String>,
    pub calories_per_serving: f64,
    pub food_type: FoodType,
    pub components: Vec<(String, f64)>, // (food_id, servings)
}

impl Food {
    pub fn new_basic(id: String, name: String, keywords: HashSet<String>, calories: f64) -> Self {
        Food {
            id,
            name,
            keywords,
            calories_per_serving: calories,
            food_type: FoodType::Basic,
            components: Vec::new(),
        }
    }

    pub fn new_composite(id: String, name: String, keywords: HashSet<String>, components: Vec<(String, f64)>) -> Self {
        Food {
            id,
            name,
            keywords,
            calories_per_serving: 0.0, // Will be calculated later based on components
            food_type: FoodType::Composite,
            components,
        }
    }

    pub fn matches_keywords(&self, search_keywords: &HashSet<String>, match_all: bool) -> bool {
        if match_all {
            search_keywords.iter().all(|k| self.keywords.contains(k))
        } else {
            search_keywords.iter().any(|k| self.keywords.contains(k))
        }
    }
}