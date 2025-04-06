

// src/models/log.rs
use chrono::{DateTime, Local, NaiveDate};
use std::collections::HashMap;

use super::food::Food;

#[derive(Debug, Clone)]
pub struct FoodEntry {
    pub food_id: String,
    pub servings: f64,
    pub timestamp: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct DailyLog {
    pub date: NaiveDate,
    pub entries: Vec<FoodEntry>,
}

impl DailyLog {
    pub fn new(date: NaiveDate) -> Self {
        DailyLog {
            date,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, food_id: String, servings: f64) {
        let entry = FoodEntry {
            food_id,
            servings,
            timestamp: Local::now(),
        };
        self.entries.push(entry);
    }

    pub fn remove_entry(&mut self, index: usize) -> Option<FoodEntry> {
        if index < self.entries.len() {
            Some(self.entries.remove(index))
        } else {
            None
        }
    }

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