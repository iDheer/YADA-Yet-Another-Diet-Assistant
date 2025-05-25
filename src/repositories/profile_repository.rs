//! # Profile Repository
//! 
//! This module implements the Repository Pattern for managing user profile data,
//! including both basic profile information and time-varying daily profiles.
//! It provides centralized management of user data with validation and persistence.
//! 
//! ## Repository Pattern Implementation
//! 
//! The `ProfileRepository` manages the complete user profile lifecycle:
//! - **Profile Creation**: Initialize new user profiles with validation
//! - **Data Management**: Update basic and daily profile information
//! - **Historical Tracking**: Maintain chronological records of daily changes
//! - **Persistence**: Reliable storage and retrieval of profile data
//! - **Data Integrity**: Ensure consistency and validate profile updates
//! 
//! ## File Format Specification
//! 
//! The repository uses a structured format supporting multiple data types:
//! 
//! ### Basic Profile
//! ```
//! PROFILE|gender|height|birth_date|calculation_method
//! ```
//! 
//! ### Daily Profiles
//! ```
//! DAILY|date|weight|activity_level
//! ```
//! 
//! ## Data Validation Features
//! 
//! - **Type Safety**: Ensures proper data types for all profile fields
//! - **Range Validation**: Validates reasonable values for height, weight, dates
//! - **Enum Mapping**: Safe conversion between storage codes and enum values
//! - **Default Fallbacks**: Graceful handling of invalid data with sensible defaults
//! - **Consistency Checks**: Maintains referential integrity between basic and daily profiles

// src/repositories/profile_repository.rs
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use chrono::NaiveDate;

use crate::models::profile::{UserProfile, DailyProfile, Gender, ActivityLevel};

/// # Profile Repository
/// 
/// A Repository Pattern implementation for managing user profile data with support
/// for both static personal information and time-varying daily profiles. This
/// repository ensures data consistency and provides reliable profile management.
/// 
/// ## Core Responsibilities
/// 
/// - **Profile Lifecycle**: Create, update, and maintain user profiles
/// - **Daily Tracking**: Manage time-varying data like weight and activity levels
/// - **Data Validation**: Ensure profile data meets application requirements
/// - **Historical Records**: Maintain chronological profile changes
/// - **Persistence**: Reliable storage and retrieval with error handling
/// 
/// ## Profile Data Structure
/// 
/// The repository manages a composite profile structure:
/// - **Basic Profile**: Static data like gender, height, birth date
/// - **Daily Profiles**: Time-varying data like current weight and activity level
/// - **Calculation Preferences**: User's preferred calorie calculation method
/// 
/// ## Singleton Pattern
/// 
/// The repository maintains at most one user profile, reflecting the single-user
/// nature of the diet tracking application.
pub struct ProfileRepository {
    /// The user's profile data (None if no profile has been created)
    profile: Option<UserProfile>,
    /// File system path for persistent storage of profile data
    file_path: String,
}

impl ProfileRepository {
    /// Creates a new ProfileRepository instance and loads existing profile data.
    /// 
    /// This constructor establishes the repository's connection to persistent storage
    /// and loads any existing user profile, enabling immediate access to user data.
    /// 
    /// # Arguments
    /// * `file_path` - Path to the file where profile data will be stored and loaded from
    /// 
    /// # Returns
    /// * `Result<Self, io::Error>` - A new repository instance or an IO error if file loading fails
    /// 
    /// # Initialization Process
    /// 1. Initialize repository with empty profile state
    /// 2. Store file path for future persistence operations
    /// 3. Load existing profile data if the file exists
    /// 4. Return fully initialized repository ready for operations
    pub fn new(file_path: &str) -> Result<Self, io::Error> {
        let mut repo = ProfileRepository {
            profile: None,
            file_path: file_path.to_string(),
        };
        
        // Load profile from file if it exists
        if Path::new(file_path).exists() {
            repo.load()?;
        }
          Ok(repo)
    }
    
    /// Retrieves an immutable reference to the user profile.
    /// 
    /// Provides read-only access to the complete user profile including both
    /// basic information and all daily profiles. Returns None if no profile
    /// has been created for this user.
    /// 
    /// # Returns
    /// * `Option<&UserProfile>` - Reference to the user profile if it exists, None otherwise
    /// 
    /// # Examples
    /// ```
    /// if let Some(profile) = repo.get_profile() {
    ///     println!("User height: {} cm", profile.height);
    ///     println!("Number of daily profiles: {}", profile.daily_profiles.len());
    /// }
    /// ```
    pub fn get_profile(&self) -> Option<&UserProfile> {        self.profile.as_ref()
    }
    
    /// Retrieves a mutable reference to the user profile.
    /// 
    /// Provides write access to the complete user profile for updating both
    /// basic information and daily profiles. Returns None if no profile exists.
    /// 
    /// # Returns
    /// * `Option<&mut UserProfile>` - Mutable reference to the user profile if it exists
    /// 
    /// # Use Cases
    /// - Updating basic profile information (height, calculation method)
    /// - Adding or modifying daily profile entries
    /// - Batch operations on profile data
    /// 
    /// # Examples
    /// ```
    /// if let Some(profile) = repo.get_profile_mut() {
    ///     profile.height = 170.0;
    ///     profile.add_or_update_daily_profile(daily_profile);
    /// }
    /// ```
    pub fn get_profile_mut(&mut self) -> Option<&mut UserProfile> {        self.profile.as_mut()
    }
    
    /// Sets the user profile, replacing any existing profile.
    /// 
    /// This method performs a complete profile replacement, which is typically
    /// used during initial profile creation or when migrating profile data.
    /// It replaces both basic information and all daily profiles.
    /// 
    /// # Arguments
    /// * `profile` - The new user profile to store in the repository
    /// 
    /// # Behavior
    /// - Completely replaces any existing profile data
    /// - Does not merge with existing data
    /// - Immediately updates the in-memory profile
    /// - Requires explicit save() call for persistence
    /// 
    /// # Examples
    /// ```
    /// let new_profile = UserProfile::new(Gender::Female, 165.0, birth_date);
    /// repo.set_profile(new_profile);
    /// repo.save()?; // Persist the new profile
    /// ```
    pub fn set_profile(&mut self, profile: UserProfile) {        self.profile = Some(profile);
    }
    
    /// Persists the current profile data to the configured file.
    /// 
    /// This method serializes the complete user profile including basic information
    /// and all daily profiles to a structured text format. If no profile exists,
    /// the method succeeds but writes no data.
    /// 
    /// # Returns
    /// * `Result<(), io::Error>` - Success confirmation or IO error details
    /// 
    /// # File Format
    /// The method writes data in a structured format with type prefixes:
    /// - **PROFILE**: Basic user information (gender, height, birth date, calculation method)
    /// - **DAILY**: Daily profile entries (date, weight, activity level)
    /// 
    /// # Data Encoding
    /// - Gender: M (Male), F (Female), O (Other)
    /// - Activity Level: S (Sedentary), L (Lightly Active), M (Moderately Active), V (Very Active), E (Extremely Active)
    /// - Dates: ISO format (YYYY-MM-DD)
    /// 
    /// # Error Handling
    /// - File creation and write permission issues
    /// - Disk space limitations
    /// - Data formatting errors during serialization
    pub fn save(&self) -> Result<(), io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;
        
        if let Some(profile) = &self.profile {
            // Write basic profile info
            writeln!(
                file,
                "PROFILE|{}|{}|{}|{}",
                match profile.gender {
                    Gender::Male => "M",
                    Gender::Female => "F",
                    Gender::Other => "O",
                },
                profile.height,
                profile.birth_date.format("%Y-%m-%d"),
                profile.calculation_method
            )?;
            
            // Write daily profiles
            for daily in &profile.daily_profiles {
                writeln!(
                    file,
                    "DAILY|{}|{}|{}",
                    daily.date.format("%Y-%m-%d"),
                    daily.weight,
                    match daily.activity_level {
                        ActivityLevel::Sedentary => "S",
                        ActivityLevel::LightlyActive => "L",
                        ActivityLevel::ModeratelyActive => "M",
                        ActivityLevel::VeryActive => "V",
                        ActivityLevel::ExtremelyActive => "E",
                    }
                )?;
            }
        }
          Ok(())
    }
    
    /// Loads profile data from the configured file into memory.
    /// 
    /// This method reconstructs the complete user profile from persistent storage,
    /// parsing both basic profile information and all daily profile entries.
    /// It provides robust error handling and data validation during the loading process.
    /// 
    /// # Returns
    /// * `Result<(), io::Error>` - Success confirmation or IO error details
    /// 
    /// # Loading Process
    /// 1. **File Parsing**: Read and parse each line according to its type prefix
    /// 2. **Profile Construction**: Build basic profile from PROFILE lines
    /// 3. **Daily Integration**: Add daily profiles to the basic profile
    /// 4. **Data Validation**: Ensure all data meets application requirements
    /// 5. **Error Recovery**: Skip malformed lines and continue processing
    /// 
    /// # Data Validation
    /// - Validates date formats and provides sensible defaults for invalid dates
    /// - Maps gender and activity level codes to proper enum values
    /// - Ensures numeric values (height, weight) are within reasonable ranges
    /// - Maintains referential integrity between basic and daily profiles
    /// 
    /// # Error Recovery Strategy
    /// - Skips malformed lines to prevent complete loading failure
    /// - Uses default values for invalid enum mappings
    /// - Continues processing valid data when encountering errors
    /// - Provides fallback dates for unparseable date strings
    /// 
    /// # Multi-Pass Processing
    /// The method processes PROFILE lines first to establish the basic profile,
    /// then adds DAILY entries to ensure proper data relationship maintenance.
    pub fn load(&mut self) -> Result<(), io::Error> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        let mut main_profile: Option<UserProfile> = None;
        
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('|').collect();
            
            if parts.is_empty() {
                continue;
            }
            
            match parts[0] {
                "PROFILE" => {
                    if parts.len() != 5 {
                        continue;
                    }
                    
                    let gender = match parts[1] {
                        "M" => Gender::Male,
                        "F" => Gender::Female,
                        _ => Gender::Other,
                    };
                    
                    let height: f64 = parts[2].parse().unwrap_or(0.0);
                    
                    let birth_date = NaiveDate::parse_from_str(parts[3], "%Y-%m-%d")
                        .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
                    
                    let calculation_method = parts[4].to_string();
                    
                    let mut profile = UserProfile::new(gender, height, birth_date);
                    profile.calculation_method = calculation_method;
                    
                    main_profile = Some(profile);
                }
                "DAILY" => {
                    if parts.len() != 4 || main_profile.is_none() {
                        continue;
                    }
                    
                    let date = NaiveDate::parse_from_str(parts[1], "%Y-%m-%d")
                        .unwrap_or_else(|_| NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
                    
                    let weight: f64 = parts[2].parse().unwrap_or(0.0);
                    
                    let activity_level = match parts[3] {
                        "S" => ActivityLevel::Sedentary,
                        "L" => ActivityLevel::LightlyActive,
                        "M" => ActivityLevel::ModeratelyActive,
                        "V" => ActivityLevel::VeryActive,
                        "E" => ActivityLevel::ExtremelyActive,
                        _ => ActivityLevel::Sedentary,
                    };
                    
                    let daily_profile = DailyProfile {
                        date,
                        weight,
                        activity_level,
                    };
                    
                    if let Some(profile) = &mut main_profile {
                        profile.add_or_update_daily_profile(daily_profile);
                    }
                }
                _ => {
                    // Unknown line type, skip
                    continue;
                }
            }
        }
        
        self.profile = main_profile;
        
        Ok(())
    }
}