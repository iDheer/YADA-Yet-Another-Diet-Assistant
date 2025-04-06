// src/models/command.rs
use std::fmt;

#[derive(Debug)]
pub enum CommandType {
    AddFood,
    RemoveFood,
    AddLog,
    DeleteLog,
    UpdateProfile,
    Other(String),
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandType::AddFood => write!(f, "Add Food"),
            CommandType::RemoveFood => write!(f, "Remove Food"),
            CommandType::AddLog => write!(f, "Add Log Entry"),
            CommandType::DeleteLog => write!(f, "Delete Log Entry"),
            CommandType::UpdateProfile => write!(f, "Update Profile"),
            CommandType::Other(s) => write!(f, "{}", s),
        }
    }
}

pub trait Command {
    fn execute(&mut self) -> Result<(), String>;
    fn undo(&mut self) -> Result<(), String>;
    fn get_type(&self) -> CommandType;
    fn description(&self) -> String;
}