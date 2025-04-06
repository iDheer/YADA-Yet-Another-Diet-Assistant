// src/commands/log_commands.rs
use chrono::NaiveDate;

use crate::models::command::{Command, CommandType};
use crate::models::log::FoodEntry;
use crate::repositories::log_repository::LogRepository;

pub struct AddLogEntryCommand {
    log_repo: *mut LogRepository,
    date: NaiveDate,
    food_id: String,
    servings: f64,
    executed: bool,
}

// Note: We need to implement Send + Sync manually because of the raw pointer
unsafe impl Send for AddLogEntryCommand {}
unsafe impl Sync for AddLogEntryCommand {}

impl AddLogEntryCommand {
    pub fn new(log_repo: &mut LogRepository, date: NaiveDate, food_id: String, servings: f64) -> Self {
        AddLogEntryCommand {
            log_repo: log_repo as *mut LogRepository,
            date,
            food_id,
            servings,
            executed: false,
        }
    }
}

impl Command for AddLogEntryCommand {
    fn execute(&mut self) -> Result<(), String> {
        // Safety: We know the pointer is valid because it was created from a reference
        let log_repo = unsafe { &mut *self.log_repo };
        
        let log = log_repo.get_log_mut(self.date);
        log.add_entry(self.food_id.clone(), self.servings);
        
        self.executed = true;
        Ok(())
    }

    fn undo(&mut self) -> Result<(), String> {
        if !self.executed {
            return Err("Command was not executed".to_string());
        }

        // Safety: We know the pointer is valid because it was created from a reference
        let log_repo = unsafe { &mut *self.log_repo };
        
        // Remove the last entry for this food
        let log = log_repo.get_log_mut(self.date);
        
        // Find the entry matching our food_id (in reverse order to remove the most recent)
        for i in (0..log.entries.len()).rev() {
            if log.entries[i].food_id == self.food_id {
                log.remove_entry(i);
                break;
            }
        }
        
        self.executed = false;
        Ok(())
    }

    fn get_type(&self) -> CommandType {
        CommandType::AddLog
    }

    fn description(&self) -> String {
        format!("Add log entry: {} servings of {} on {}", 
                self.servings, self.food_id, self.date.format("%Y-%m-%d"))
    }
}

pub struct RemoveLogEntryCommand {
    log_repo: *mut LogRepository,
    date: NaiveDate,
    index: usize,
    removed_entry: Option<FoodEntry>,
    executed: bool,
}

// Note: We need to implement Send + Sync manually because of the raw pointer
unsafe impl Send for RemoveLogEntryCommand {}
unsafe impl Sync for RemoveLogEntryCommand {}

impl RemoveLogEntryCommand {
    pub fn new(log_repo: &mut LogRepository, date: NaiveDate, index: usize) -> Self {
        RemoveLogEntryCommand {
            log_repo: log_repo as *mut LogRepository,
            date,
            index,
            removed_entry: None,
            executed: false,
        }
    }
}

impl Command for RemoveLogEntryCommand {
    fn execute(&mut self) -> Result<(), String> {
        // Safety: We know the pointer is valid because it was created from a reference
        let log_repo = unsafe { &mut *self.log_repo };
        
        let log = log_repo.get_log_mut(self.date);
        
        // Remove the entry at the specified index
        if let Some(entry) = log.remove_entry(self.index) {
            self.removed_entry = Some(entry);
            self.executed = true;
            Ok(())
        } else {
            Err(format!("No entry at index {} to remove", self.index))
        }
    }

    fn undo(&mut self) -> Result<(), String> {
        if !self.executed {
            return Err("Command was not executed".to_string());
        }

        // Safety: We know the pointer is valid because it was created from a reference
        let log_repo = unsafe { &mut *self.log_repo };
        
        let log = log_repo.get_log_mut(self.date);
        
        // Restore the removed entry
        if let Some(entry) = &self.removed_entry {
            log.entries.insert(self.index, entry.clone());
            self.executed = false;
            Ok(())
        } else {
            Err("No entry to restore".to_string())
        }
    }

    fn get_type(&self) -> CommandType {
        CommandType::DeleteLog
    }

    fn description(&self) -> String {
        if let Some(entry) = &self.removed_entry {
            format!("Remove log entry: {} servings of {} on {}", 
                    entry.servings, entry.food_id, self.date.format("%Y-%m-%d"))
        } else {
            format!("Remove log entry at index {} on {}", 
                    self.index, self.date.format("%Y-%m-%d"))
        }
    }
}
