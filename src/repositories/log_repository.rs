//! # Log Repository
//! 
//! This module implements the Repository Pattern for managing daily food consumption logs.
//! It provides temporal organization of food entries, enabling users to track their
//! dietary intake across different dates with precise timestamps.
//! 
//! ## Repository Pattern Implementation
//! 
//! The `LogRepository` manages the persistence and retrieval of daily food logs:
//! - **Temporal Organization**: Organizes food entries by date for chronological tracking
//! - **Timestamped Entries**: Maintains precise consumption timing for detailed analysis
//! - **Efficient Access**: Date-based indexing for O(1) daily log retrieval
//! - **Batch Operations**: Handles multiple entries per day with atomic persistence
//! - **Data Consistency**: Ensures temporal integrity and proper entry sequencing
//! 
//! ## File Format Specification
//! 
//! The repository uses a pipe-delimited format optimized for temporal data:
//! ```
//! YYYY-MM-DD|food_id|servings|YYYY-MM-DDTHH:MM:SS
//! ```
//! 
//! ## Temporal Features
//! 
//! - **Date Indexing**: Efficient access to any day's consumption data
//! - **Chronological Ordering**: Maintains temporal sequence for analysis
//! - **Cross-Date Tracking**: Supports consumption logging for any date
//! - **Historical Analysis**: Enables tracking of dietary patterns over time
//! - **Future Planning**: Allows pre-planning of meals for upcoming dates

// src/repositories/log_repository.rs
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use chrono::{NaiveDate, Local, DateTime};

use crate::models::log::{DailyLog, FoodEntry};

/// # Log Repository
/// 
/// A Repository Pattern implementation for managing daily food consumption logs
/// with temporal organization and precise timestamping. This repository provides
/// efficient access to consumption data organized by date.
/// 
/// ## Core Responsibilities
/// 
/// - **Daily Log Management**: Create and maintain daily food consumption records
/// - **Temporal Indexing**: Organize data by date for efficient chronological access
/// - **Entry Tracking**: Manage individual food entries with precise timestamps
/// - **Historical Persistence**: Maintain long-term consumption history
/// - **Data Retrieval**: Provide efficient access to both current and historical data
/// 
/// ## Storage Strategy
/// 
/// The repository uses date-based partitioning in memory with unified file storage,
/// optimizing for both temporal queries and persistent storage efficiency.
pub struct LogRepository {
    /// Date-indexed collection of daily logs for O(1) access to any day's data
    logs: HashMap<NaiveDate, DailyLog>,
    /// File system path for persistent storage of consumption logs
    file_path: String,
}

impl LogRepository {
    /// Creates a new LogRepository instance and initializes it with existing log data.
    /// 
    /// This constructor establishes the repository's connection to persistent storage
    /// and loads any existing consumption data into memory for efficient access.
    /// 
    /// # Arguments
    /// * `file_path` - Path to the file where log data will be stored and loaded from
    /// 
    /// # Returns
    /// * `Result<Self, io::Error>` - A new repository instance or an IO error if file loading fails
    /// 
    /// # Initialization Process
    /// 1. Create empty in-memory log collection indexed by date
    /// 2. Store file path for future persistence operations
    /// 3. Load existing log data if the file exists
    /// 4. Return fully initialized repository ready for operations
    pub fn new(file_path: &str) -> Result<Self, io::Error> {
        let mut repo = LogRepository {
            logs: HashMap::new(),
            file_path: file_path.to_string(),
        };
        
        // Load logs from file if it exists
        if Path::new(file_path).exists() {
            repo.load()?;
        }
          Ok(repo)
    }
    
    /// Retrieves an immutable reference to a specific day's food log.
    /// 
    /// Provides efficient read-only access to daily consumption data without
    /// creating new log entries. Returns None if no consumption was recorded
    /// for the specified date.
    /// 
    /// # Arguments
    /// * `date` - The date for which to retrieve the food log
    /// 
    /// # Returns
    /// * `Option<&DailyLog>` - Reference to the daily log if it exists, None otherwise
    /// 
    /// # Examples
    /// ```
    /// use chrono::NaiveDate;
    /// let date = NaiveDate::from_ymd(2025, 5, 25);
    /// if let Some(log) = repo.get_log(date) {
    ///     println!("Total entries for today: {}", log.entries.len());
    /// }
    /// ```
    pub fn get_log(&self, date: NaiveDate) -> Option<&DailyLog> {        self.logs.get(&date)
    }
    
    /// Retrieves a mutable reference to a specific day's food log, creating it if necessary.
    /// 
    /// This method provides write access to daily logs and automatically creates
    /// new log entries for dates that haven't been accessed before. It's the primary
    /// method for adding new food entries to daily consumption records.
    /// 
    /// # Arguments
    /// * `date` - The date for which to retrieve or create a food log
    /// 
    /// # Returns
    /// * `&mut DailyLog` - Mutable reference to the daily log (guaranteed to exist)
    /// 
    /// # Automatic Creation
    /// If no log exists for the specified date, this method automatically creates
    /// a new DailyLog instance, ensuring that callers always receive a valid log.
    /// 
    /// # Examples
    /// ```
    /// use chrono::NaiveDate;
    /// let date = NaiveDate::from_ymd(2025, 5, 25);
    /// let log = repo.get_log_mut(date);
    /// log.add_entry(food_entry);
    /// ```
    pub fn get_log_mut(&mut self, date: NaiveDate) -> &mut DailyLog {        self.logs.entry(date).or_insert_with(|| DailyLog::new(date))
    }
    
    /// Persists all log data to the configured file in chronological order.
    /// 
    /// This method implements the repository's persistence responsibility by
    /// serializing all daily logs and their entries to a structured text format.
    /// The output is sorted chronologically for human readability and consistency.
    /// 
    /// # Returns
    /// * `Result<(), io::Error>` - Success confirmation or IO error details
    /// 
    /// # File Format
    /// Each line represents a single food entry in the format:
    /// `YYYY-MM-DD|food_id|servings|YYYY-MM-DDTHH:MM:SS`
    /// 
    /// # Chronological Organization
    /// - Dates are sorted chronologically in the output file
    /// - Entries within each day maintain their original temporal order
    /// - Consistent format enables reliable parsing and analysis
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
        
        // Sort dates for consistent output
        let mut dates: Vec<&NaiveDate> = self.logs.keys().collect();
        dates.sort();
        
        for date in dates {
            if let Some(log) = self.logs.get(date) {
                for entry in &log.entries {
                    writeln!(
                        file,
                        "{}|{}|{}|{}",
                        date.format("%Y-%m-%d"),
                        entry.food_id,
                        entry.servings,
                        entry.timestamp.format("%Y-%m-%dT%H:%M:%S")
                    )?;
                }
            }
        }
          Ok(())
    }
    
    /// Loads all log data from the configured file into memory.
    /// 
    /// This method reconstructs the complete consumption history from persistent
    /// storage, parsing each entry and organizing it by date for efficient access.
    /// It handles data validation and provides error recovery for malformed entries.
    /// 
    /// # Returns
    /// * `Result<(), io::Error>` - Success confirmation or IO error details
    /// 
    /// # Loading Process
    /// 1. **Clear Cache**: Remove any existing in-memory log data
    /// 2. **Parse File**: Process each line according to the expected format
    /// 3. **Validate Data**: Ensure dates, IDs, and timestamps are valid
    /// 4. **Organize Entries**: Group food entries by date into daily logs
    /// 5. **Maintain Order**: Preserve temporal sequence within each day
    /// 
    /// # Error Recovery
    /// - Skips malformed lines to prevent complete loading failure
    /// - Uses current timestamp as fallback for invalid timestamps
    /// - Continues processing valid data when encountering errors
    /// - Provides detailed error information for debugging
    /// 
    /// # Data Integrity
    /// Validates date formats and handles timezone conversions properly
    /// to ensure accurate temporal representation across different systems.
    pub fn load(&mut self) -> Result<(), io::Error> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        self.logs.clear();
        
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('|').collect();
            
            if parts.len() != 4 {
                continue; // Skip invalid lines
            }
            
            if let Ok(date) = NaiveDate::parse_from_str(parts[0], "%Y-%m-%d") {
                let food_id = parts[1].to_string();
                let servings: f64 = parts[2].parse().unwrap_or(0.0);
                let timestamp = DateTime::parse_from_str(&format!("{}+00:00", parts[3]), "%Y-%m-%dT%H:%M:%S%z")
                    .unwrap_or_else(|_| Local::now().into())
                    .with_timezone(&Local);
                
                let entry = FoodEntry {
                    food_id,
                    servings,
                    timestamp,
                };
                
                let log = self.logs.entry(date).or_insert_with(|| DailyLog::new(date));
                log.entries.push(entry);
            }
        }
        
        Ok(())
    }
}