//! # Repositories Module
//! 
//! This module implements the **Repository Pattern** to provide clean separation between 
//! business logic and data access concerns. The repository pattern abstracts the data 
//! storage mechanism and provides a consistent interface for CRUD operations across 
//! different data types.
//!
//! ## Design Pattern: Repository Pattern
//! 
//! The Repository Pattern provides several key benefits:
//! - **Separation of Concerns**: Business logic is decoupled from data access
//! - **Testability**: Easy to mock repositories for unit testing
//! - **Consistency**: Uniform interface across all data access operations
//! - **Flexibility**: Can switch storage mechanisms without changing business logic
//! - **Centralized Queries**: All data access logic is centralized in repositories
//!
//! ## Repository Architecture
//! 
//! Each repository follows a consistent structure:
//! - **Creation**: Factory methods for initialization with file path configuration
//! - **CRUD Operations**: Create, Read, Update, Delete operations for data entities
//! - **Querying**: Search and filter operations with domain-specific logic
//! - **Persistence**: Save and load operations for file-based storage
//! - **Error Handling**: Robust error management with detailed error messages
//!
//! ## File-Based Storage Strategy
//! 
//! All repositories use a simple, human-readable text file format:
//! - **Portability**: Files can be easily backed up, shared, or migrated
//! - **Debugging**: File contents are readable for troubleshooting
//! - **Simplicity**: No external database dependencies required
//! - **Version Control**: Text files work well with version control systems
//!
//! ## Module Organization
//! 
//! - `food_repository`: Manages the food database with composite pattern support
//! - `log_repository`: Handles daily food consumption logs with temporal organization
//! - `profile_repository`: Manages user profile data with validation and history

// Repository modules for data persistence (Repository Pattern implementation)
pub mod food_repository;
pub mod log_repository;
pub mod profile_repository;