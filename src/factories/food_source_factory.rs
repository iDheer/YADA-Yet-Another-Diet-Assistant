// src/factories/food_source_factory.rs
use std::collections::HashMap;

use crate::models::food::Food;

pub trait FoodSource {
    fn get_food_by_id(&self, id: &str) -> Option<Food>;
    fn search_foods(&self, query: &str) -> Vec<Food>;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

pub struct FoodSourceFactory {
    sources: HashMap<String, Box<dyn FoodSource>>,
}

impl FoodSourceFactory {
    pub fn new() -> Self {
        let mut factory = FoodSourceFactory {
            sources: HashMap::new(),
        };
        
        // Register built-in sources
        factory.register_source(Box::new(LocalFoodSource {}));
        
        factory
    }
    
    pub fn register_source(&mut self, source: Box<dyn FoodSource>) {
        self.sources.insert(source.name().to_string(), source);
    }
    
    pub fn get_source(&self, name: &str) -> Option<&Box<dyn FoodSource>> {
        self.sources.get(name)
    }
    
    pub fn get_all_sources(&self) -> Vec<&str> {
        self.sources.keys().map(|s| s.as_str()).collect()
    }
}

// A simple local food source that doesn't actually do anything
// This is just a placeholder to show how the factory pattern would work
struct LocalFoodSource {}

impl FoodSource for LocalFoodSource {
    fn get_food_by_id(&self, _id: &str) -> Option<Food> {
        None
    }
    
    fn search_foods(&self, _query: &str) -> Vec<Food> {
        Vec::new()
    }
    
    fn name(&self) -> &'static str {
        "local"
    }
    
    fn description(&self) -> &'static str {
        "Local food database"
    }
}

// In a real application, you might have implementations like:
// - USDAFoodSource that connects to the USDA food database API
// - McDonaldsSource that scrapes McDonald's nutrition information
// - etc.

// Example of how a third-party API food source might look:
/*
struct USDAFoodSource {
    api_key: String,
    client: USDAClient,
}

impl FoodSource for USDAFoodSource {
    fn get_food_by_id(&self, id: &str) -> Option<Food> {
        match self.client.get_food_details(id) {
            Ok(details) => {
                let mut keywords = HashSet::new();
                keywords.insert(details.name.to_lowercase());
                for category in &details.categories {
                    keywords.insert(category.to_lowercase());
                }
                
                Some(Food::new_basic(
                    format!("usda_{}", id),
                    details.name,
                    keywords,
                    details.calories_per_100g / 100.0, // Convert to calories per 1g
                ))
            },
            Err(_) => None,
        }
    }
    
    fn search_foods(&self, query: &str) -> Vec<Food> {
        match self.client.search_foods(query) {
            Ok(results) => {
                results.iter().filter_map(|item| self.get_food_by_id(&item.id)).collect()
            },
            Err(_) => Vec::new(),
        }
    }
    
    fn name(&self) -> &'static str {
        "usda"
    }
    
    fn description(&self) -> &'static str {
        "USDA Food Database"
    }
}
*/