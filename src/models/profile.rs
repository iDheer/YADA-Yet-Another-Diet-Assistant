// src/models/profile.rs
use chrono::NaiveDate;
use chrono::Datelike;  // Add this import for the year() and with_year() methods

#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActivityLevel {
    Sedentary,
    LightlyActive,
    ModeratelyActive,
    VeryActive,
    ExtremelyActive,
}

#[derive(Debug, Clone)]
pub struct DailyProfile {
    pub date: NaiveDate,
    pub weight: f64, // in kg
    pub activity_level: ActivityLevel,
}

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub gender: Gender,
    pub height: f64, // in cm
    pub birth_date: NaiveDate,
    pub daily_profiles: Vec<DailyProfile>,
    pub calculation_method: String, // Identifies which calorie calculation method to use
}

impl UserProfile {
    pub fn new(gender: Gender, height: f64, birth_date: NaiveDate) -> Self {
        UserProfile {
            gender,
            height,
            birth_date,
            daily_profiles: Vec::new(),
            calculation_method: "harris_benedict".to_string(), // Default
        }
    }

    pub fn age(&self, as_of_date: NaiveDate) -> u32 {
        let years = as_of_date.year() - self.birth_date.year();
        let birth_day_in_current_year = self.birth_date
            .with_year(as_of_date.year())
            .unwrap_or(self.birth_date);
        
        if birth_day_in_current_year > as_of_date {
            years as u32 - 1
        } else {
            years as u32
        }
    }

    pub fn get_daily_profile(&self, date: NaiveDate) -> Option<&DailyProfile> {
        self.daily_profiles.iter().find(|&p| p.date == date)
    }

    pub fn add_or_update_daily_profile(&mut self, profile: DailyProfile) {
        if let Some(idx) = self.daily_profiles.iter().position(|p| p.date == profile.date) {
            self.daily_profiles[idx] = profile;
        } else {
            self.daily_profiles.push(profile);
        }
    }
}