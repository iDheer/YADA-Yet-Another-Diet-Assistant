//! # Commands Module
//! 
//! This module implements the **Command Pattern** to encapsulate all data modification
//! operations as executable objects with full undo capability. Each command represents
//! a specific business operation that can be executed, undone, and provides detailed
//! information about its effects.
//! 
//! ## Design Pattern: Command Pattern
//! 
//! The Command Pattern provides several critical benefits:
//! - **Encapsulation**: Each operation is wrapped in a self-contained command object
//! - **Undo/Redo Support**: Every command knows how to reverse its effects
//! - **Logging**: Commands provide detailed information about operations performed
//! - **Queuing**: Commands can be stored, delayed, or batched for execution
//! - **Macro Commands**: Complex operations can be built from simpler command compositions
//! 
//! ## Command Architecture
//! 
//! All commands implement the `Command` trait with the following responsibilities:
//! - **execute()**: Perform the primary operation and return detailed results
//! - **undo()**: Reverse the operation's effects and restore previous state
//! - **description()**: Provide human-readable operation description
//! - **Error Handling**: Robust error management with detailed error messages
//! 
//! ## Command Categories
//! 
//! Commands are organized by the domain they operate on:
//! - **Food Commands**: Manage food database operations (add, update, remove foods)
//! - **Log Commands**: Handle daily consumption tracking (add, remove log entries)
//! - **Profile Commands**: Manage user profile data (basic profile, daily updates)
//! 
//! ## Undo System Integration
//! 
//! Each command maintains the necessary state information to reverse its effects:
//! - **State Capture**: Commands store previous state before making modifications
//! - **Atomic Operations**: Commands ensure either complete success or complete rollback
//! - **Dependency Management**: Commands handle complex object relationships during undo
//! - **Validation**: Commands verify system state before and after operations
//! 
//! ## Module Organization
//! 
//! - `food_commands`: Food database manipulation commands
//! - `log_commands`: Daily consumption log management commands  
//! - `profile_commands`: User profile modification commands

// Command pattern implementations for all data modification operations
pub mod food_commands;
pub mod log_commands;
pub mod profile_commands;