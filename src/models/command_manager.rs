//! Command Manager - Central Command Pattern Orchestration
//! 
//! This module implements the command execution and undo management system
//! for the YADA application. It provides centralized control over all
//! data-modifying operations with robust undo functionality.
//! 
//! ## Key Features:
//! - Command execution with automatic undo stack management
//! - Configurable undo stack size with automatic cleanup
//! - Command history tracking for audit and display purposes
//! - Error handling for both execution and undo operations
//! - Memory management to prevent unlimited command accumulation
//! 
//! ## Design Benefits:
//! - Centralized command execution ensures consistent behavior
//! - Automatic undo stack management simplifies client code
//! - Bounded memory usage prevents command history from growing indefinitely
//! - Type-safe command handling through trait objects

// src/models/command_manager.rs
use crate::models::command::Command;

/// Central manager for command execution and undo functionality
/// 
/// CommandManager provides the core infrastructure for the Command Pattern
/// implementation in YADA. It manages:
/// - Command execution with automatic success tracking
/// - Undo stack maintenance with configurable size limits
/// - Command history for user interface and debugging
/// - Memory management to prevent unbounded growth
/// 
/// ## Undo Stack Management:
/// Only successfully executed commands are added to the undo stack.
/// The stack has a configurable maximum size, with oldest commands
/// automatically removed when the limit is exceeded.
pub struct CommandManager {
    /// Stack of successfully executed commands available for undo
    undo_stack: Vec<Box<dyn Command>>,
    
    /// Maximum number of commands to keep in undo history
    max_stack_size: usize,
}

impl CommandManager {
    /// Creates a new CommandManager with specified undo stack size limit
    /// 
    /// # Arguments
    /// * `max_stack_size` - Maximum number of commands to retain for undo
    /// 
    /// # Examples
    /// ```
    /// let manager = CommandManager::new(50); // Keep last 50 commands
    /// ```
    pub fn new(max_stack_size: usize) -> Self {
        CommandManager {
            undo_stack: Vec::new(),
            max_stack_size,
        }
    }
    
    /// Executes a command and manages undo stack automatically
    /// 
    /// This method:
    /// 1. Attempts to execute the provided command
    /// 2. On success, adds the command to the undo stack
    /// 3. Manages stack size by removing oldest commands if needed
    /// 4. On failure, discards the command (no undo stack modification)
    /// 
    /// # Arguments
    /// * `command` - Boxed command object implementing the Command trait
    /// 
    /// # Returns
    /// * `Ok(())` - Command executed successfully and added to undo stack
    /// * `Err(String)` - Command execution failed with error description
    pub fn execute_command(&mut self, mut command: Box<dyn Command>) -> Result<(), String> {
        let result = command.execute();
        
        if result.is_ok() {
            // Add to undo stack
            self.undo_stack.push(command);
            
            // If we've exceeded the max stack size, remove the oldest command
            if self.undo_stack.len() > self.max_stack_size {
                self.undo_stack.remove(0);
            }
        }
        
        result
    }
    
    /// Undoes the most recently executed command
    /// 
    /// This method:
    /// 1. Removes the most recent command from the undo stack
    /// 2. Calls the command's undo() method to reverse its effects
    /// 3. Permanently removes the command from undo history
    /// 
    /// Note: Once undone, commands cannot be redone (no redo stack)
    /// 
    /// # Returns
    /// * `Ok(())` - Command undone successfully
    /// * `Err(String)` - No commands to undo or undo operation failed
    pub fn undo_last_command(&mut self) -> Result<(), String> {
        if let Some(mut command) = self.undo_stack.pop() {
            command.undo()
        } else {
            Err("No command to undo".to_string())
        }
    }
    
    /// Returns the current number of commands available for undo
    /// 
    /// Useful for user interface elements that show undo availability
    /// or for debugging and monitoring purposes.
    /// 
    /// # Returns
    /// Number of commands currently in the undo stack
    pub fn get_undo_stack_size(&self) -> usize {
        self.undo_stack.len()
    }
    
    /// Checks whether any commands are available for undo
    /// 
    /// This is a convenience method for user interface logic that
    /// needs to enable/disable undo functionality.
    /// 
    /// # Returns
    /// * `true` - At least one command is available for undo
    /// * `false` - No commands available for undo
    pub fn has_commands_to_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    
    /// Generates a list of command descriptions for history display
    /// 
    /// This method creates a human-readable command history by collecting
    /// the description() from each command in the undo stack. Useful for:
    /// - Displaying command history to users
    /// - Debugging and audit trails
    /// - Undo preview functionality
    /// 
    /// # Returns
    /// Vector of strings describing each command in chronological order
    /// (oldest commands first, newest commands last)
    pub fn get_command_history(&self) -> Vec<String> {
        self.undo_stack
            .iter()
            .map(|cmd| cmd.description())
            .collect()
    }
}
