// src/commands/profile_commands.rs
use crate::models::command::{Command, CommandType};
use crate::models::profile::{UserProfile, DailyProfile};
use crate::repositories::profile_repository::ProfileRepository;

pub struct UpdateUserProfileCommand {
    profile_repo: *mut ProfileRepository,
    old_profile: Option<UserProfile>,
    new_profile: UserProfile,
    executed: bool,
}

// Note: We need to implement Send + Sync manually because of the raw pointer
unsafe impl Send for UpdateUserProfileCommand {}
unsafe impl Sync for UpdateUserProfileCommand {}

impl UpdateUserProfileCommand {
    pub fn new(profile_repo: &mut ProfileRepository, new_profile: UserProfile) -> Self {
        let old_profile = profile_repo.get_profile().cloned();
        
        UpdateUserProfileCommand {
            profile_repo: profile_repo as *mut ProfileRepository,
            old_profile,
            new_profile,
            executed: false,
        }
    }
}

impl Command for UpdateUserProfileCommand {
    fn execute(&mut self) -> Result<(), String> {
        // Safety: We know the pointer is valid because it was created from a reference
        let profile_repo = unsafe { &mut *self.profile_repo };
        
        profile_repo.set_profile(self.new_profile.clone());
        self.executed = true;
        Ok(())
    }

    fn undo(&mut self) -> Result<(), String> {
        if !self.executed {
            return Err("Command was not executed".to_string());
        }

        // Safety: We know the pointer is valid because it was created from a reference
        let profile_repo = unsafe { &mut *self.profile_repo };
        
        // Restore the old profile if it exists
        if let Some(old_profile) = &self.old_profile {
            profile_repo.set_profile(old_profile.clone());
        } else {
            // No previous profile existed
            profile_repo.set_profile(UserProfile::new(
                self.new_profile.gender.clone(),
                self.new_profile.height,
                self.new_profile.birth_date,
            ));
        }
        
        self.executed = false;
        Ok(())
    }

    fn get_type(&self) -> CommandType {
        CommandType::UpdateProfile
    }

    fn description(&self) -> String {
        "Update user profile".to_string()
    }
}

pub struct UpdateDailyProfileCommand {
    profile_repo: *mut ProfileRepository,
    daily_profile: DailyProfile,
    old_daily_profile: Option<DailyProfile>,
    executed: bool,
}

// Note: We need to implement Send + Sync manually because of the raw pointer
unsafe impl Send for UpdateDailyProfileCommand {}
unsafe impl Sync for UpdateDailyProfileCommand {}

impl UpdateDailyProfileCommand {
    pub fn new(profile_repo: &mut ProfileRepository, daily_profile: DailyProfile) -> Self {
        let old_daily_profile = profile_repo
            .get_profile()
            .and_then(|p| p.get_daily_profile(daily_profile.date))
            .cloned();
        
        UpdateDailyProfileCommand {
            profile_repo: profile_repo as *mut ProfileRepository,
            daily_profile,
            old_daily_profile,
            executed: false,
        }
    }
}

impl Command for UpdateDailyProfileCommand {
    fn execute(&mut self) -> Result<(), String> {
        // Safety: We know the pointer is valid because it was created from a reference
        let profile_repo = unsafe { &mut *self.profile_repo };
        
        if let Some(profile) = profile_repo.get_profile_mut() {
            profile.add_or_update_daily_profile(self.daily_profile.clone());
            self.executed = true;
            Ok(())
        } else {
            Err("No user profile exists".to_string())
        }
    }

    fn undo(&mut self) -> Result<(), String> {
        if !self.executed {
            return Err("Command was not executed".to_string());
        }

        // Safety: We know the pointer is valid because it was created from a reference
        let profile_repo = unsafe { &mut *self.profile_repo };
        
        if let Some(profile) = profile_repo.get_profile_mut() {
            // If we have an old daily profile, restore it
            if let Some(old_daily) = &self.old_daily_profile {
                profile.add_or_update_daily_profile(old_daily.clone());
            } else {
                // Otherwise remove the daily profile
                profile.daily_profiles.retain(|p| p.date != self.daily_profile.date);
            }
            
            self.executed = false;
            Ok(())
        } else {
            Err("No user profile exists".to_string())
        }
    }

    fn get_type(&self) -> CommandType {
        CommandType::UpdateProfile
    }

    fn description(&self) -> String {
        format!("Update daily profile for {}", self.daily_profile.date.format("%Y-%m-%d"))
    }
}
