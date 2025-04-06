// src/repositories/profile_repository.rs
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use chrono::NaiveDate;

use crate::models::profile::{UserProfile, DailyProfile, Gender, ActivityLevel};

pub struct ProfileRepository {
    profile: Option<UserProfile>,
    file_path: String,
}

impl ProfileRepository {
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
    
    pub fn get_profile(&self) -> Option<&UserProfile> {
        self.profile.as_ref()
    }
    
    pub fn get_profile_mut(&mut self) -> Option<&mut UserProfile> {
        self.profile.as_mut()
    }
    
    pub fn set_profile(&mut self, profile: UserProfile) {
        self.profile = Some(profile);
    }
    
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