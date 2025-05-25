//! # Food Repository
//! 
//! This module implements the data access layer for the food database using the Repository Pattern.
//! It provides a clean interface for managing food data persistence while supporting both basic 
//! and composite foods through the Composite Pattern.
//! 
//! ## Repository Pattern Implementation
//! 
//! The `FoodRepository` abstracts all food-related data operations:
//! - **CRUD Operations**: Add, update, retrieve, and manage food entities
//! - **Search Functionality**: Keyword-based searching with AND/OR logic
//! - **File Persistence**: Save and load operations for durable storage
//! - **Composite Food Support**: Handles recursive calorie calculations for recipes
//! - **Error Management**: Comprehensive error handling for all operations
//! 
//! ## File Format Specification
//! 
//! The repository uses a pipe-delimited text format for data storage:
//! 
//! ### Basic Foods
//! ```
//! B|food_id|food_name|keyword1,keyword2,keyword3|calories_per_serving
//! ```
//! 
//! ### Composite Foods
//! ```
//! C|food_id|food_name|keyword1,keyword2,keyword3|component1:servings1,component2:servings2
//! ```
//! 
//! ## Data Integrity Features
//! 
//! - **Duplicate Prevention**: Enforces unique food IDs across the database
//! - **Dependency Management**: Validates composite food components exist
//! - **Recursive Calculation**: Automatically updates composite food calories
//! - **Error Recovery**: Graceful handling of malformed data entries
//! - **Consistency Checks**: Ensures data integrity during load operations

// src/repositories/food_repository.rs
use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use crate::models::food::{Food, FoodType};

/// # Food Repository
/// 
/// A Repository Pattern implementation that manages the persistent storage and retrieval 
/// of food data. This repository supports both basic and composite foods, providing a 
/// clean interface for all food-related data operations.
/// 
/// ## Core Responsibilities
/// 
/// - **Data Management**: Add, update, and retrieve food entities with validation
/// - **Search Operations**: Keyword-based food discovery with flexible matching
/// - **File Persistence**: Durable storage using human-readable text format
/// - **Composite Food Support**: Automatic calorie calculation for recipes
/// - **Data Integrity**: Ensures consistency and validates food dependencies
/// 
/// ## Storage Format
/// 
/// The repository uses a pipe-delimited format optimized for both human readability 
/// and programmatic parsing, supporting the full spectrum of food types in the system.
pub struct FoodRepository {
    /// In-memory cache of all foods, indexed by unique food ID for O(1) access
    foods: HashMap<String, Food>,
    /// File system path for persistent storage of food data
    file_path: String,
}

impl FoodRepository {
    /// Creates a new FoodRepository instance and initializes it with data from the specified file.
    /// 
    /// This constructor implements the Repository Pattern by providing a clean interface for
    /// data access while hiding the underlying storage implementation details.
    /// 
    /// # Arguments
    /// * `file_path` - Path to the file where food data will be stored and loaded from
    /// 
    /// # Returns
    /// * `Result<Self, io::Error>` - A new repository instance or an IO error if file loading fails
    /// 
    /// # Examples
    /// ```
    /// let repo = FoodRepository::new("foods.txt")?;
    /// ```
    /// 
    /// # Workflow
    /// 1. Initialize empty food cache and store file path
    /// 2. Check if data file exists in the file system
    /// 3. If file exists, load all food data into memory
    /// 4. Return fully initialized repository ready for operations
    pub fn new(file_path: &str) -> Result<Self, io::Error> {
        let mut repo = FoodRepository {
            foods: HashMap::new(),
            file_path: file_path.to_string(),
        };
        
        // Load foods from file if it exists
        if Path::new(file_path).exists() {
            repo.load()?;
        }
          Ok(repo)
    }
    
    /// Adds a new food to the repository with duplicate detection.
    /// 
    /// This method enforces data integrity by preventing duplicate food IDs and validates
    /// that the food entity meets all repository requirements before insertion.
    /// 
    /// # Arguments
    /// * `food` - The food entity to add to the repository
    /// 
    /// # Returns
    /// * `Result<(), String>` - Success confirmation or detailed error message
    /// 
    /// # Errors
    /// * Returns error if a food with the same ID already exists
    /// 
    /// # Examples
    /// ```
    /// let apple = Food::new_basic("apple".to_string(), "Apple".to_string(), 
    ///                           vec!["fruit".to_string()], 52.0);
    /// repo.add_food(apple)?;
    /// ```
    pub fn add_food(&mut self, food: Food) -> Result<(), String> {
        if self.foods.contains_key(&food.id) {
            return Err(format!("Food with ID {} already exists", food.id));
        }
          self.foods.insert(food.id.clone(), food);
        Ok(())
    }
    
    /// Updates an existing food in the repository with validation.
    /// 
    /// This method modifies an existing food entity while maintaining data integrity
    /// and ensuring that all references to the food remain valid.
    /// 
    /// # Arguments
    /// * `food` - The updated food entity with the same ID as the existing food
    /// 
    /// # Returns
    /// * `Result<(), String>` - Success confirmation or detailed error message
    /// 
    /// # Errors
    /// * Returns error if no food exists with the specified ID
    /// 
    /// # Note
    /// This operation affects composite foods that reference the updated food,
    /// requiring calorie recalculation for dependent recipes.
    pub fn update_food(&mut self, food: Food) -> Result<(), String> {
        if !self.foods.contains_key(&food.id) {
            return Err(format!("Food with ID {} not found", food.id));
        }
          self.foods.insert(food.id.clone(), food);
        Ok(())
    }
    
    /// Retrieves a food by its unique identifier.
    /// 
    /// Provides O(1) access to food entities through the internal HashMap index,
    /// supporting efficient lookups for both display and calculation operations.
    /// 
    /// # Arguments
    /// * `id` - The unique identifier of the food to retrieve
    /// 
    /// # Returns
    /// * `Option<&Food>` - A reference to the food if found, None otherwise
    /// 
    /// # Examples
    /// ```
    /// if let Some(apple) = repo.get_food("apple") {
    ///     println!("Calories: {}", apple.calories_per_serving);
    /// }
    /// ```
    pub fn get_food(&self, id: &str) -> Option<&Food> {        self.foods.get(id)
    }
    
    /// Returns all foods in the repository as a vector of references.
    /// 
    /// Provides access to the complete food database for operations like
    /// browsing, bulk processing, or generating comprehensive reports.
    /// 
    /// # Returns
    /// * `Vec<&Food>` - A vector containing references to all foods in the repository
    /// 
    /// # Performance
    /// This operation creates a new vector but uses references to avoid copying
    /// food data, making it efficient for read-only operations.
    pub fn get_all_foods(&self) -> Vec<&Food> {        self.foods.values().collect()
    }
    
    /// Searches for foods based on keyword matching with configurable logic.
    /// 
    /// Implements flexible search functionality supporting both AND and OR logic
    /// for keyword matching, enabling users to find foods with varying levels
    /// of specificity in their search criteria.
    /// 
    /// # Arguments
    /// * `keywords` - Set of keywords to search for in food keywords
    /// * `match_all` - If true, uses AND logic (all keywords must match); if false, uses OR logic
    /// 
    /// # Returns
    /// * `Vec<&Food>` - Vector of food references matching the search criteria
    /// 
    /// # Search Logic
    /// - **AND Logic**: Food must contain ALL specified keywords
    /// - **OR Logic**: Food must contain AT LEAST ONE specified keyword
    /// 
    /// # Examples
    /// ```
    /// // Find foods that are both "fruit" AND "sweet"
    /// let keywords = HashSet::from(["fruit".to_string(), "sweet".to_string()]);
    /// let results = repo.search_foods(&keywords, true);
    /// 
    /// // Find foods that are either "fruit" OR "vegetable"
    /// let results = repo.search_foods(&keywords, false);
    /// ```
    pub fn search_foods(&self, keywords: &HashSet<String>, match_all: bool) -> Vec<&Food> {
        self.foods
            .values()
            .filter(|food| food.matches_keywords(keywords, match_all))            .collect()
    }
    
    /// Persists all food data to the configured file using a structured format.
    /// 
    /// Implements the repository's persistence responsibility by serializing all
    /// food entities to a human-readable, parseable text format. The method handles
    /// both basic and composite foods with their respective data requirements.
    /// 
    /// # Returns
    /// * `Result<(), io::Error>` - Success confirmation or IO error details
    /// 
    /// # File Format
    /// - **Basic Foods**: `B|id|name|keywords|calories`
    /// - **Composite Foods**: `C|id|name|keywords|component1:servings1,component2:servings2`
    /// 
    /// # Error Handling
    /// - File creation failures
    /// - Write permission issues
    /// - Disk space limitations
    /// 
    /// # Data Integrity
    /// The method uses truncate mode to ensure clean writes and prevent
    /// data corruption from partial write operations.
    pub fn save(&self) -> Result<(), io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;
        
        for food in self.foods.values() {
            let keywords = food.keywords.iter().cloned().collect::<Vec<_>>().join(",");
            
            match food.food_type {
                FoodType::Basic => {
                    writeln!(
                        file,
                        "B|{}|{}|{}|{}",
                        food.id,
                        food.name,
                        keywords,
                        food.calories_per_serving
                    )?;
                }
                FoodType::Composite => {
                    let components = food
                        .components
                        .iter()
                        .map(|(id, servings)| format!("{}:{}", id, servings))
                        .collect::<Vec<_>>()
                        .join(",");
                    
                    writeln!(
                        file,
                        "C|{}|{}|{}|{}",
                        food.id,
                        food.name,
                        keywords,
                        components
                    )?;
                }
            }
        }
          Ok(())
    }
    
    /// Loads all food data from the configured file into memory.
    /// 
    /// This method implements the repository's data loading responsibility,
    /// parsing the structured text format and reconstructing food entities
    /// in memory. It handles both basic and composite foods with proper
    /// dependency resolution and calorie calculation.
    /// 
    /// # Returns
    /// * `Result<(), io::Error>` - Success confirmation or IO error details
    /// 
    /// # Loading Process
    /// 1. **Clear Cache**: Remove any existing in-memory food data
    /// 2. **Parse File**: Process each line according to food type format
    /// 3. **Create Entities**: Construct Food objects from parsed data
    /// 4. **Resolve Dependencies**: Calculate composite food calories
    /// 5. **Validate Integrity**: Ensure all food references are valid
    /// 
    /// # Error Recovery
    /// - Skips malformed lines rather than failing completely
    /// - Continues processing valid data when encountering errors
    /// - Provides detailed error information for debugging
    /// 
    /// # Composite Food Handling
    /// Uses a two-pass approach to ensure all component foods are loaded
    /// before calculating composite food calorie values.
    pub fn load(&mut self) -> Result<(), io::Error> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        self.foods.clear();
        
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('|').collect();
            
            if parts.len() < 4 {
                continue; // Skip invalid lines
            }
            
            match parts[0] {
                "B" => {
                    // Basic food format: B|id|name|keywords|calories
                    if parts.len() != 5 {
                        continue;
                    }
                    
                    let id = parts[1].to_string();
                    let name = parts[2].to_string();
                    let keywords = parts[3]
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                    let calories: f64 = parts[4].parse().unwrap_or(0.0);
                    
                    let food = Food::new_basic(id.clone(), name, keywords, calories);
                    self.foods.insert(id, food);
                }
                "C" => {
                    // Composite food format: C|id|name|keywords|component1:servings1,component2:servings2,...
                    if parts.len() != 5 {
                        continue;
                    }
                    
                    let id = parts[1].to_string();
                    let name = parts[2].to_string();
                    let keywords = parts[3]
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                    
                    let components = parts[4]
                        .split(',')
                        .filter_map(|comp| {
                            let comp_parts: Vec<&str> = comp.split(':').collect();
                            if comp_parts.len() != 2 {
                                return None;
                            }
                            
                            let comp_id = comp_parts[0].to_string();
                            let servings: f64 = comp_parts[1].parse().unwrap_or(0.0);
                            Some((comp_id, servings))
                        })
                        .collect();
                    
                    let mut food = Food::new_composite(id.clone(), name, keywords, components);
                    
                    // Calculate calories based on components
                    let mut total_calories = 0.0;
                    for (comp_id, servings) in &food.components {
                        if let Some(component) = self.foods.get(comp_id) {
                            total_calories += component.calories_per_serving * servings;
                        }
                    }
                    food.calories_per_serving = total_calories;
                    
                    self.foods.insert(id, food);
                }
                _ => {
                    // Skip unknown food types
                    continue;
                }
            }
        }
        
        // Recalculate calories for all composite foods
        // (need to do this after loading all foods to ensure dependencies are loaded)
        let food_ids: Vec<String> = self.foods
            .values()
            .filter(|f| matches!(f.food_type, FoodType::Composite))
            .map(|f| f.id.clone())
            .collect();
        
        for id in food_ids {
            if let Some(food) = self.foods.get(&id) {
                if let FoodType::Composite = food.food_type {
                    let mut total_calories = 0.0;
                    
                    for (comp_id, servings) in &food.components {
                        if let Some(component) = self.foods.get(comp_id) {
                            total_calories += component.calories_per_serving * servings;
                        }
                    }
                    
                    if let Some(food) = self.foods.get_mut(&id) {
                        food.calories_per_serving = total_calories;
                    }
                }
            }
        }
          Ok(())
    }

    /// Provides mutable access to the internal food HashMap for advanced operations.
    /// 
    /// This method exposes the internal data structure for operations that require
    /// direct manipulation of the food collection, such as batch updates or
    /// complex data transformations that aren't covered by standard CRUD operations.
    /// 
    /// # Returns
    /// * `&mut HashMap<String, Food>` - Mutable reference to the internal food storage
    /// 
    /// # Use Cases
    /// - Batch operations that modify multiple foods
    /// - Complex data migrations or transformations
    /// - Advanced search operations requiring custom logic
    /// 
    /// # Warning
    /// Direct manipulation of the HashMap bypasses repository validation,
    /// so callers must ensure data integrity when using this method.
    pub fn get_foods_mut(&mut self) -> &mut HashMap<String, Food> {
        &mut self.foods
    }
    
    /// Provides immutable access to the internal food HashMap for read operations.
    /// 
    /// This method allows efficient read-only access to the complete food collection
    /// without the overhead of creating new data structures, supporting advanced
    /// querying and analysis operations.
    /// 
    /// # Returns
    /// * `&HashMap<String, Food>` - Immutable reference to the internal food storage
    /// 
    /// # Use Cases
    /// - Performance-critical read operations
    /// - Complex queries requiring HashMap iteration
    /// - Statistical analysis of the food database
    /// - Custom search algorithms
    /// 
    /// # Benefits
    /// - O(1) access to individual foods by ID
    /// - Efficient iteration over the entire collection
    /// - No data copying overhead for large operations
    pub fn get_foods(&self) -> &HashMap<String, Food> {
        &self.foods
    }
}