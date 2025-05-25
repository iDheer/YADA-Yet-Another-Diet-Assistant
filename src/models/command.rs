//! Command Pattern Implementation for Undo/Redo Functionality
//! 
//! This module defines the core Command Pattern interface used throughout
//! the YADA application to enable undo/redo functionality for all data
//! modification operations.
//! 
//! ## Design Pattern: Command Pattern
//! The Command Pattern encapsulates operations as objects, allowing:
//! - Parameterization of clients with different requests
//! - Queuing of operations for later execution
//! - Logging of operations for audit trails
//! - Undo/redo functionality through command reversal
//! 
//! ## Supported Operations:
//! All data-modifying operations in YADA implement this Command interface,
//! including food management, logging, and profile updates.

// src/models/command.rs
use std::fmt;

/// Enumeration of all supported command types in the application
/// 
/// CommandType provides categorization for different kinds of operations
/// that can be executed and undone. This enables:
/// - Type-safe command identification
/// - User-friendly command descriptions
/// - Command filtering and analysis
/// - Audit trail categorization
#[derive(Debug)]
pub enum CommandType {
    /// Adding new food items to the database
    AddFood,
    
    /// Removing food items from the database
    RemoveFood,
    
    /// Adding new entries to food logs
    AddLog,
    
    /// Deleting entries from food logs
    DeleteLog,
    
    /// Updating user profile information
    UpdateProfile,
    
    /// Extensible category for future command types
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

/// Core Command trait defining the Command Pattern interface
/// 
/// All data-modifying operations in YADA must implement this trait to enable:
/// - Consistent execution semantics across all operations
/// - Reliable undo functionality for all commands
/// - Command categorization and description
/// - Error handling with descriptive messages
/// 
/// ## Implementation Requirements:
/// - `execute()`: Perform the forward operation
/// - `undo()`: Reverse the operation completely
/// - `get_type()`: Return the command category
/// - `description()`: Provide human-readable command description
/// 
/// ## Error Handling:
/// Both execute() and undo() return Result<(), String> to provide
/// descriptive error messages when operations fail.
pub trait Command {
    /// Executes the command's forward operation
    /// 
    /// This method performs the intended operation (add, remove, update, etc.).
    /// Must be idempotent - calling multiple times should be safe.
    /// 
    /// # Returns
    /// * `Ok(())` - Operation completed successfully
    /// * `Err(String)` - Operation failed with descriptive error message
    fn execute(&mut self) -> Result<(), String>;
    
    /// Reverses the command's operation (undo functionality)
    /// 
    /// This method must completely reverse the effects of execute().
    /// Should restore the system to the exact state before execute() was called.
    /// 
    /// # Returns
    /// * `Ok(())` - Undo completed successfully
    /// * `Err(String)` - Undo failed with descriptive error message
    fn undo(&mut self) -> Result<(), String>;
    
    /// Returns the type/category of this command
    /// 
    /// Used for command classification, filtering, and user interface display.
    /// 
    /// # Returns
    /// CommandType enum value identifying the operation category
    fn get_type(&self) -> CommandType;
    
    /// Provides a human-readable description of the command
    /// 
    /// Used for command history display, undo confirmations, and audit logs.
    /// Should include relevant details like affected items or quantities.
    /// 
    /// # Returns
    /// String describing what this command does (e.g., "Add apple to food database")
    fn description(&self) -> String;
}