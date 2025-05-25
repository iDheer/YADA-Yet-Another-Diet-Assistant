//! Models Module - Core Data Structures and Business Logic
//! 
//! This module contains the fundamental data models and business logic for the YADA application.
//! It implements several key design patterns:
//! 
//! ## Design Patterns Implemented:
//! - **Composite Pattern**: Food model supports both basic and composite foods
//! - **Command Pattern**: Command trait and command manager for undo functionality
//! - **Repository Pattern**: Data models designed for repository abstraction
//! 
//! ## Module Organization:
//! - `food`: Food entities with support for basic and composite food types
//! - `log`: Daily food consumption logging with date-based organization
//! - `profile`: User profile management with basic and daily profile components
//! - `command`: Command trait definition for the Command Pattern implementation
//! - `command_manager`: Command execution and undo management system

// src/models/mod.rs
pub mod food;
pub mod log;
pub mod profile;
pub mod command;
pub mod command_manager;
