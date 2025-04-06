// src/repositories/food_repository.rs
use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use crate::models::food::{Food, FoodType};

pub struct FoodRepository {
    foods: HashMap<String, Food>,
    file_path: String,
}

impl FoodRepository {
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
    
    pub fn add_food(&mut self, food: Food) -> Result<(), String> {
        if self.foods.contains_key(&food.id) {
            return Err(format!("Food with ID {} already exists", food.id));
        }
        
        self.foods.insert(food.id.clone(), food);
        Ok(())
    }
    
    pub fn update_food(&mut self, food: Food) -> Result<(), String> {
        if !self.foods.contains_key(&food.id) {
            return Err(format!("Food with ID {} not found", food.id));
        }
        
        self.foods.insert(food.id.clone(), food);
        Ok(())
    }
    
    pub fn get_food(&self, id: &str) -> Option<&Food> {
        self.foods.get(id)
    }
    
    pub fn get_all_foods(&self) -> Vec<&Food> {
        self.foods.values().collect()
    }
    
    pub fn search_foods(&self, keywords: &HashSet<String>, match_all: bool) -> Vec<&Food> {
        self.foods
            .values()
            .filter(|food| food.matches_keywords(keywords, match_all))
            .collect()
    }
    
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

    // Get a mutable reference to the foods HashMap
    pub fn get_foods_mut(&mut self) -> &mut HashMap<String, Food> {
        &mut self.foods
    }
    
    // Get an immutable reference to the foods HashMap
    pub fn get_foods(&self) -> &HashMap<String, Food> {
        &self.foods
    }
}