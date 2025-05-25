//! Food Log Model - Daily Food Consumption Tracking
//! 
//! This module implements the data structures for tracking daily food consumption.
//! It provides a date-based organization system for logging food entries with
//! precise timestamps and serving amounts.
//! 
//! ## Key Features:
//! - Date-based log organization for daily tracking
//! - Timestamped food entries for chronological ordering
//! - Flexible serving amounts (not limited to whole servings)
//! - Calorie calculation integration with food database
//! - Entry management (add/remove) with index-based operations
//! 
//! ## Integration:
//! Works seamlessly with the Food model and Repository pattern to provide
//! comprehensive food consumption tracking and calorie analysis.

// src/models/log.rs
use chrono::{DateTime, Local, NaiveDate};
use std::collections::HashMap;

use super::food::Food;

/// Individual food consumption entry with timing and quantity information
/// 
/// Each FoodEntry represents a single instance of food consumption, containing:
/// - Reference to the consumed food (via food_id)
/// - Amount consumed in servings (supports fractional amounts)
/// - Precise timestamp for chronological tracking
/// 
/// This granular approach enables detailed analysis of eating patterns
/// and accurate calorie tracking throughout the day.
#[derive(Debug, Clone)]
pub struct FoodEntry {
    /// References a food item in the food database
    pub food_id: String,
    
    /// Amount consumed (supports fractional servings like 0.5, 1.5, etc.)
    pub servings: f64,
    
    /// Exact time when the food was logged (enables chronological analysis)
    pub timestamp: DateTime<Local>,
}

/// Daily food consumption log containing all entries for a specific date
/// 
/// DailyLog organizes food consumption by date, providing:
/// - Date-based organization for daily tracking
/// - Collection of all food entries for the day
/// - Methods for entry management and calorie calculations
/// - Integration with food database for nutritional analysis
/// 
/// This structure supports the application's daily tracking workflow
/// and enables comprehensive nutritional analysis and reporting.
#[derive(Debug, Clone)]
pub struct DailyLog {
    /// The date for which this log tracks food consumption
    pub date: NaiveDate,
    
    /// All food entries logged for this date (chronologically ordered)
    pub entries: Vec<FoodEntry>,
}

impl DailyLog {
    /// Creates a new empty daily log for the specified date
    /// 
    /// # Arguments
    /// * `date` - The date for which this log will track food consumption
    /// 
    /// # Returns
    /// A new DailyLog instance with no entries
    pub fn new(date: NaiveDate) -> Self {
        DailyLog {
            date,
            entries: Vec::new(),
        }
    }

    /// Adds a new food entry to the daily log with current timestamp
    /// 
    /// This method creates and appends a new FoodEntry to the log:
    /// 1. Creates entry with current timestamp for chronological tracking
    /// 2. Appends to entries vector maintaining chronological order
    /// 3. Supports fractional servings for precise quantity tracking
    /// 
    /// # Arguments
    /// * `food_id` - Reference to a food item in the food database
    /// * `servings` - Amount consumed (supports fractions like 0.5, 1.5)
    /// 
    /// # Examples
    /// ```
    /// log.add_entry("apple".to_string(), 1.0);     // One apple
    /// log.add_entry("bread".to_string(), 0.5);     // Half serving of bread
    /// ```
    pub fn add_entry(&mut self, food_id: String, servings: f64) {
        let entry = FoodEntry {
            food_id,
            servings,
            timestamp: Local::now(),
        };
        self.entries.push(entry);
    }

    /// Removes a food entry from the log by index position
    /// 
    /// This method enables deletion of specific food entries:
    /// 1. Validates index bounds to prevent panics
    /// 2. Removes entry and returns it for potential undo operations
    /// 3. Maintains chronological order of remaining entries
    /// 
    /// # Arguments
    /// * `index` - Zero-based index of the entry to remove
    /// 
    /// # Returns
    /// * `Some(FoodEntry)` - The removed entry if index was valid
    /// * `None` - If index was out of bounds
    /// 
    /// # Examples
    /// ```
    /// if let Some(removed_entry) = log.remove_entry(0) {
    ///     println!("Removed: {} servings of {}", removed_entry.servings, removed_entry.food_id);
    /// }
    /// ```
    pub fn remove_entry(&mut self, index: usize) -> Option<FoodEntry> {
        if index < self.entries.len() {
            Some(self.entries.remove(index))
        } else {
            None
        }
    }

    /// Calculates total calories consumed for the day based on food database
    /// 
    /// This method performs calorie aggregation by:
    /// 1. Iterating through all food entries for the day
    /// 2. Looking up calorie information from the food database
    /// 3. Calculating calories as: food.calories_per_serving * entry.servings
    /// 4. Summing all entry calories for daily total
    /// 
    /// # Arguments
    /// * `food_db` - HashMap containing food definitions with calorie information
    /// 
    /// # Returns
    /// Total calories consumed for the day as f64
    /// 
    /// # Note
    /// Entries referencing non-existent foods are ignored in the calculation,
    /// ensuring robust operation even with data inconsistencies.
    pub fn total_calories(&self, food_db: &HashMap<String, Food>) -> f64 {
        let mut total = 0.0;
        for entry in &self.entries {
            if let Some(food) = food_db.get(&entry.food_id) {
                total += food.calories_per_serving * entry.servings;
            }
        }
        total
    }
}