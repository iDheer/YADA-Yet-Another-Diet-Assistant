// src/strategies/calorie_calculator.rs
use chrono::NaiveDate;
use std::collections::HashMap;

use crate::models::profile::{UserProfile, ActivityLevel, Gender};

pub trait CalorieCalculator {
    fn calculate_target_calories(&self, profile: &UserProfile, date: NaiveDate) -> f64;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

pub struct CalorieCalculatorFactory {
    calculators: HashMap<String, Box<dyn CalorieCalculator>>,
}

impl CalorieCalculatorFactory {
    pub fn new() -> Self {
        let mut factory = CalorieCalculatorFactory {
            calculators: HashMap::new(),
        };
        
        // Register available calculators
        factory.register_calculator(Box::new(HarrisBenedictCalculator {}));
        factory.register_calculator(Box::new(MifflinStJeorCalculator {}));
        
        factory
    }
    
    pub fn register_calculator(&mut self, calculator: Box<dyn CalorieCalculator>) {
        self.calculators.insert(calculator.name().to_string(), calculator);
    }
    
    pub fn get_calculator(&self, name: &str) -> Option<&Box<dyn CalorieCalculator>> {
        self.calculators.get(name)
    }
    
    pub fn get_all_calculators(&self) -> Vec<&str> {
        self.calculators.keys().map(|s| s.as_str()).collect()
    }
}

// Harris-Benedict Equation
pub struct HarrisBenedictCalculator {}

impl CalorieCalculator for HarrisBenedictCalculator {
    fn calculate_target_calories(&self, profile: &UserProfile, date: NaiveDate) -> f64 {
        let daily_profile = match profile.get_daily_profile(date) {
            Some(p) => p,
            None => return 0.0, // No profile for this date
        };
        
        let age = profile.age(date);
        let height = profile.height; // cm
        let weight = daily_profile.weight; // kg
        
        // Base metabolic rate (BMR) calculation
        let bmr = match profile.gender {
            Gender::Male => 88.362 + (13.397 * weight) + (4.799 * height) - (5.677 * age as f64),
            Gender::Female => 447.593 + (9.247 * weight) + (3.098 * height) - (4.330 * age as f64),
            Gender::Other => {
                // Average of male and female equations
                let male_bmr = 88.362 + (13.397 * weight) + (4.799 * height) - (5.677 * age as f64);
                let female_bmr = 447.593 + (9.247 * weight) + (3.098 * height) - (4.330 * age as f64);
                (male_bmr + female_bmr) / 2.0
            }
        };
        
        // Apply activity factor
        let activity_multiplier = match daily_profile.activity_level {
            ActivityLevel::Sedentary => 1.2,
            ActivityLevel::LightlyActive => 1.375,
            ActivityLevel::ModeratelyActive => 1.55,
            ActivityLevel::VeryActive => 1.725,
            ActivityLevel::ExtremelyActive => 1.9,
        };
        
        bmr * activity_multiplier
    }
    
    fn name(&self) -> &'static str {
        "harris_benedict"
    }
    
    fn description(&self) -> &'static str {
        "Harris-Benedict Equation (Revised 1984)"
    }
}

// Mifflin-St Jeor Equation
pub struct MifflinStJeorCalculator {}

impl CalorieCalculator for MifflinStJeorCalculator {
    fn calculate_target_calories(&self, profile: &UserProfile, date: NaiveDate) -> f64 {
        let daily_profile = match profile.get_daily_profile(date) {
            Some(p) => p,
            None => return 0.0, // No profile for this date
        };
        
        let age = profile.age(date);
        let height = profile.height; // cm
        let weight = daily_profile.weight; // kg
        
        // Base metabolic rate (BMR) calculation
        let bmr = match profile.gender {
            Gender::Male => (10.0 * weight) + (6.25 * height) - (5.0 * age as f64) + 5.0,
            Gender::Female => (10.0 * weight) + (6.25 * height) - (5.0 * age as f64) - 161.0,
            Gender::Other => {
                // Average of male and female equations
                let male_bmr = (10.0 * weight) + (6.25 * height) - (5.0 * age as f64) + 5.0;
                let female_bmr = (10.0 * weight) + (6.25 * height) - (5.0 * age as f64) - 161.0;
                (male_bmr + female_bmr) / 2.0
            }
        };
        
        // Apply activity factor
        let activity_multiplier = match daily_profile.activity_level {
            ActivityLevel::Sedentary => 1.2,
            ActivityLevel::LightlyActive => 1.375,
            ActivityLevel::ModeratelyActive => 1.55,
            ActivityLevel::VeryActive => 1.725,
            ActivityLevel::ExtremelyActive => 1.9,
        };
        
        bmr * activity_multiplier
    }
    
    fn name(&self) -> &'static str {
        "mifflin_st_jeor"
    }
    
    fn description(&self) -> &'static str {
        "Mifflin-St Jeor Equation"
    }
}