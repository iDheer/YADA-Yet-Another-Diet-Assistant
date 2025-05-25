//! User Profile Model - Personal Information and Daily Tracking
//! 
//! This module implements user profile management with support for both static
//! personal information and dynamic daily tracking data. It enables accurate
//! calorie calculations by combining personal characteristics with daily variables.
//! 
//! ## Profile Architecture:
//! - **UserProfile**: Static information (gender, height, birth date)
//! - **DailyProfile**: Daily variables (weight, activity level)
//! - **Strategy Integration**: Calorie calculation method selection
//! 
//! ## Key Features:
//! - Age calculation accounting for leap years and birth dates
//! - Daily profile management with date-based organization
//! - Flexible activity level tracking for accurate TDEE calculations
//! - Integration with Strategy pattern for calorie calculation methods

// src/models/profile.rs
use chrono::NaiveDate;
use chrono::Datelike;  // Add this import for the year() and with_year() methods

/// User gender enumeration for biological calorie calculation differences
/// 
/// Gender affects BMR calculations as men and women have different
/// metabolic rates due to differences in muscle mass and body composition.
/// The "Other" option provides inclusivity while defaulting to gender-neutral
/// calculation methods when implemented.
#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Other,
}

/// Activity level enumeration for TDEE (Total Daily Energy Expenditure) calculations
/// 
/// Activity level significantly impacts calorie needs by multiplying BMR
/// with appropriate activity factors:
/// - Sedentary: Desk job, little/no exercise (BMR × 1.2)
/// - Lightly Active: Light exercise 1-3 days/week (BMR × 1.375)
/// - Moderately Active: Moderate exercise 3-5 days/week (BMR × 1.55)
/// - Very Active: Hard exercise 6-7 days/week (BMR × 1.725)
/// - Extremely Active: Very hard exercise, physical job (BMR × 1.9)
#[derive(Debug, Clone, PartialEq)]
pub enum ActivityLevel {
    Sedentary,
    LightlyActive,
    ModeratelyActive,
    VeryActive,
    ExtremelyActive,
}

/// Daily profile tracking weight and activity level for specific dates
/// 
/// DailyProfile enables day-to-day tracking of variables that affect
/// calorie calculations:
/// - Weight changes over time for accurate BMR calculations
/// - Activity level variations (rest days vs workout days)
/// - Date-specific data for historical tracking and analysis
/// 
/// This granular approach provides more accurate calorie targets than
/// static profile information alone.
#[derive(Debug, Clone)]
pub struct DailyProfile {
    /// Date for which this profile applies
    pub date: NaiveDate,
    
    /// Current weight in kilograms (affects BMR calculations)
    pub weight: f64,
    
    /// Activity level for this specific date (affects TDEE multiplier)
    pub activity_level: ActivityLevel,
}

/// Main user profile containing static personal information and daily tracking
/// 
/// UserProfile combines static personal characteristics with a collection
/// of daily profiles to enable accurate, personalized calorie calculations.
/// 
/// ## Static Information:
/// - Gender, height, birth date (used for BMR calculations)
/// - Calorie calculation method preference (Strategy pattern)
/// 
/// ## Dynamic Information:
/// - Collection of daily profiles (weight, activity level by date)
/// - Enables tracking changes over time for improved accuracy
#[derive(Debug, Clone)]
pub struct UserProfile {
    /// Biological gender for BMR calculation differences
    pub gender: Gender,
    
    /// Height in centimeters (static personal characteristic)
    pub height: f64,
    
    /// Birth date for accurate age calculation
    pub birth_date: NaiveDate,
    
    /// Collection of daily profiles indexed by date
    pub daily_profiles: Vec<DailyProfile>,
    
    /// Selected calorie calculation method (Strategy pattern identifier)
    pub calculation_method: String,
}

impl UserProfile {
    /// Creates a new user profile with basic personal information
    /// 
    /// Initializes a profile with static information and sets default
    /// calculation method to Harris-Benedict formula.
    /// 
    /// # Arguments
    /// * `gender` - Biological gender for BMR calculations
    /// * `height` - Height in centimeters
    /// * `birth_date` - Birth date for age calculations
    /// 
    /// # Returns
    /// New UserProfile with empty daily profiles and default calculation method
    pub fn new(gender: Gender, height: f64, birth_date: NaiveDate) -> Self {
        UserProfile {
            gender,
            height,
            birth_date,
            daily_profiles: Vec::new(),
            calculation_method: "harris_benedict".to_string(), // Default
        }
    }

    /// Calculates current age based on birth date and reference date
    /// 
    /// This method performs accurate age calculation accounting for:
    /// 1. Year difference between dates
    /// 2. Whether birthday has occurred in the current year
    /// 3. Leap year considerations for February 29th births
    /// 
    /// # Arguments
    /// * `as_of_date` - Reference date for age calculation (typically current date)
    /// 
    /// # Returns
    /// Age in complete years as of the reference date
    /// 
    /// # Examples
    /// ```
    /// let age = profile.age(NaiveDate::from_ymd(2024, 1, 15));
    /// ```
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

    /// Retrieves daily profile data for a specific date
    /// 
    /// This method searches the daily profiles collection for data
    /// matching the specified date, enabling date-specific calorie
    /// calculations and tracking.
    /// 
    /// # Arguments
    /// * `date` - The date for which to retrieve daily profile data
    /// 
    /// # Returns
    /// * `Some(&DailyProfile)` - Reference to daily profile if found
    /// * `None` - If no daily profile exists for the specified date
    /// 
    /// # Usage
    /// Used by calorie calculation strategies to get current weight
    /// and activity level for accurate TDEE calculations.
    pub fn get_daily_profile(&self, date: NaiveDate) -> Option<&DailyProfile> {
        self.daily_profiles.iter().find(|&p| p.date == date)
    }

    /// Adds new daily profile or updates existing one for the specified date
    /// 
    /// This method manages daily profile data by:
    /// 1. Searching for existing profile with matching date
    /// 2. Updating existing profile if found
    /// 3. Adding new profile if no existing entry found
    /// 
    /// This approach ensures one profile per date while allowing updates
    /// to weight and activity level throughout the day.
    /// 
    /// # Arguments
    /// * `profile` - DailyProfile containing date, weight, and activity level
    /// 
    /// # Examples
    /// ```
    /// let daily = DailyProfile {
    ///     date: today,
    ///     weight: 70.0,
    ///     activity_level: ActivityLevel::ModeratelyActive,
    /// };
    /// user_profile.add_or_update_daily_profile(daily);
    /// ```
    pub fn add_or_update_daily_profile(&mut self, profile: DailyProfile) {
        if let Some(idx) = self.daily_profiles.iter().position(|p| p.date == profile.date) {
            self.daily_profiles[idx] = profile;
        } else {
            self.daily_profiles.push(profile);
        }
    }
}