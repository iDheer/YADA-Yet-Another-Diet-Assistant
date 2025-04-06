// src/repositories/log_repository.rs
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use chrono::{NaiveDate, Local, DateTime};

use crate::models::log::{DailyLog, FoodEntry};

pub struct LogRepository {
    logs: HashMap<NaiveDate, DailyLog>,
    file_path: String,
}

impl LogRepository {
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
    
    pub fn get_log(&self, date: NaiveDate) -> Option<&DailyLog> {
        self.logs.get(&date)
    }
    
    pub fn get_log_mut(&mut self, date: NaiveDate) -> &mut DailyLog {
        self.logs.entry(date).or_insert_with(|| DailyLog::new(date))
    }
    
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