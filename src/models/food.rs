//! Food Model - Implements Composite Pattern for Food Hierarchy
//! 
//! This module defines the core food entities used throughout the YADA application.
//! It implements the Composite Pattern to handle both simple and complex food types
//! seamlessly within the same interface.
//! 
//! ## Design Pattern: Composite Pattern
//! The Food struct can represent:
//! - **Basic Foods**: Simple food items with direct calorie values (e.g., apple, bread)
//! - **Composite Foods**: Complex foods built from multiple components (e.g., sandwich, recipe)
//! 
//! This allows treating individual foods and compositions of foods uniformly,
//! enabling complex meal planning and nutritional calculations.

// src/models/food.rs
use std::collections::HashSet;

/// Enumeration defining the type of food item
/// 
/// This supports the Composite Pattern by distinguishing between:
/// - Basic: Simple food items with direct nutritional values
/// - Composite: Complex foods composed of multiple food components
#[derive(Debug, Clone, PartialEq)]
pub enum FoodType {
    Basic,
    Composite,
}

/// Core food entity implementing the Composite Pattern
/// 
/// The Food struct provides a unified interface for both basic and composite foods:
/// 
/// ## Basic Foods:
/// - Have direct calorie values (`calories_per_serving`)
/// - Empty components vector
/// - Represent simple food items (fruits, vegetables, basic ingredients)
/// 
/// ## Composite Foods:
/// - Calorie value calculated from components
/// - Components vector contains (food_id, servings) pairs
/// - Represent complex foods (recipes, meals, prepared dishes)
/// 
/// ## Search Functionality:
/// Both food types support keyword-based searching with AND/OR logic
/// for flexible food discovery and management.
#[derive(Debug, Clone)]
pub struct Food {
    /// Unique identifier for the food item (no spaces, used for lookups)
    pub id: String,
    
    /// Human-readable name for display purposes
    pub name: String,
    
    /// Set of lowercase keywords for search functionality
    pub keywords: HashSet<String>,
    
    /// Calories per serving (direct for basic foods, calculated for composite)
    pub calories_per_serving: f64,
    
    /// Type indicator for Composite Pattern implementation
    pub food_type: FoodType,
    
    /// Components for composite foods: (food_id, serving_amount) pairs
    pub components: Vec<(String, f64)>,
}

impl Food {
    /// Creates a new basic food item with direct calorie specification
    /// 
    /// Basic foods represent simple food items that have known nutritional
    /// values and don't need to be broken down into components.
    /// 
    /// # Arguments
    /// * `id` - Unique identifier (no spaces)
    /// * `name` - Display name for the food
    /// * `keywords` - Search keywords (should be lowercase)
    /// * `calories` - Direct calorie value per serving
    /// 
    /// # Examples
    /// ```
    /// let apple = Food::new_basic(
    ///     "apple".to_string(),
    ///     "Apple".to_string(),
    ///     keywords,
    ///     95.0
    /// );
    /// ```
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

    /// Creates a new composite food item built from existing food components
    /// 
    /// Composite foods implement the Composite Pattern by allowing complex foods
    /// to be built from simpler components. The calorie value is calculated
    /// automatically based on the components and their serving amounts.
    /// 
    /// # Arguments
    /// * `id` - Unique identifier (no spaces)
    /// * `name` - Display name for the composite food
    /// * `keywords` - Search keywords (should be lowercase)
    /// * `components` - Vector of (food_id, servings) pairs that make up this food
    /// 
    /// # Examples
    /// ```
    /// let sandwich = Food::new_composite(
    ///     "sandwich".to_string(),
    ///     "Ham Sandwich".to_string(),
    ///     keywords,
    ///     vec![("bread".to_string(), 2.0), ("ham".to_string(), 1.0)]
    /// );
    /// ```
    /// 
    /// Note: The calories_per_serving is initially set to 0.0 and should be
    /// calculated by the application logic based on component calories.
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

    /// Performs keyword-based search matching with flexible AND/OR logic
    /// 
    /// This method enables flexible food searching by allowing users to specify
    /// whether all keywords must match (AND logic) or any keyword can match (OR logic).
    /// 
    /// # Arguments
    /// * `search_keywords` - Set of keywords to search for (should be lowercase)
    /// * `match_all` - If true, ALL search keywords must be found (AND logic)
    ///                 If false, ANY search keyword match is sufficient (OR logic)
    /// 
    /// # Returns
    /// * `true` if the food matches the search criteria
    /// * `false` if the food doesn't match the search criteria
    /// 
    /// # Examples
    /// ```
    /// // AND search: food must have both "fruit" AND "red" keywords
    /// let matches_and = food.matches_keywords(&search_terms, true);
    /// 
    /// // OR search: food must have either "fruit" OR "red" keyword
    /// let matches_or = food.matches_keywords(&search_terms, false);
    /// ```
    pub fn matches_keywords(&self, search_keywords: &HashSet<String>, match_all: bool) -> bool {
        if match_all {
            // AND logic: all search keywords must be present in food keywords
            search_keywords.iter().all(|k| self.keywords.contains(k))
        } else {
            // OR logic: any search keyword match is sufficient
            search_keywords.iter().any(|k| self.keywords.contains(k))
        }
    }
}