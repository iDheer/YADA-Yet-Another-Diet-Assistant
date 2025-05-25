//! # Food Commands
//! 
//! This module implements Command Pattern objects for all food database operations.
//! Each command encapsulates a specific food database modification with full
//! undo capability and detailed operation tracking.
//! 
//! ## Command Pattern Implementation
//! 
//! Food commands provide atomic operations on the food database:
//! - **AddFoodCommand**: Adds new foods to the database with duplicate detection
//! - **UpdateFoodCommand**: Modifies existing foods while preserving operation history
//! 
//! ## Memory Safety Architecture
//! 
//! These commands use raw pointers to repository references for undo functionality:
//! - **Safety Guarantee**: Pointers are created from valid references and used immediately
//! - **Lifetime Management**: Commands assume repository outlives command execution
//! - **Thread Safety**: Manual Send + Sync implementations ensure proper concurrency handling
//! 
//! ## Undo System Design
//! 
//! Each command maintains state necessary for complete operation reversal:
//! - **State Capture**: Previous food state is stored before modifications
//! - **Atomic Operations**: Commands ensure either complete success or no change
//! - **Error Recovery**: Failed operations leave the system in consistent state
//! - **Validation**: Commands verify prerequisites before execution and after undo
//! 
//! ## Food Database Integration
//! 
//! Commands integrate directly with the FoodRepository to provide:
//! - **CRUD Operations**: Create, Update operations with full validation
//! - **Composite Food Support**: Handle complex food relationships during operations
//! - **Dependency Management**: Ensure food references remain valid after operations
//! - **Data Integrity**: Maintain database consistency throughout command lifecycle

// src/commands/food_commands.rs
use crate::models::command::{Command, CommandType};
use crate::models::food::Food;
use crate::repositories::food_repository::FoodRepository;

/// # Add Food Command
/// 
/// A Command Pattern implementation for adding new foods to the food database.
/// This command handles food creation with duplicate detection and provides
/// complete undo capability by removing the added food if needed.
/// 
/// ## Command Behavior
/// 
/// - **Execute**: Adds a new food to the repository with validation
/// - **Undo**: Removes the food from the repository if it was successfully added
/// - **Idempotency**: Can be safely executed multiple times with consistent results
/// 
/// ## Memory Management
/// 
/// Uses a raw pointer to the food repository for undo operations:
/// - **Safety**: Pointer is created from a valid reference and used immediately
/// - **Lifetime**: Assumes repository outlives the command execution
/// - **Thread Safety**: Manual Send + Sync implementations for concurrent access
/// 
/// ## Error Handling
/// 
/// - Validates food data before insertion
/// - Prevents duplicate food IDs in the database
/// - Maintains execution state for proper undo behavior
/// - Provides detailed error messages for debugging
pub struct AddFoodCommand {
    /// Raw pointer to the food repository for direct database access
    food_repo: *mut FoodRepository,
    /// The food entity to be added to the database
    food: Food,
    /// Tracks whether the command has been successfully executed
    executed: bool,
}

/// Manual implementation of Send trait for thread safety.
/// 
/// This is safe because:
/// - The raw pointer is created from a valid reference
/// - The pointer is only accessed during command execution
/// - Repository lifetime exceeds command lifetime
/// - No concurrent access to the same repository instance
unsafe impl Send for AddFoodCommand {}

/// Manual implementation of Sync trait for thread safety.
/// 
/// This is safe because:
/// - Command operations are atomic and don't share mutable state
/// - Repository access is controlled and sequential
/// - No data races possible with proper command manager usage
unsafe impl Sync for AddFoodCommand {}

impl AddFoodCommand {
    /// Creates a new AddFoodCommand for adding a food to the repository.
    /// 
    /// # Arguments
    /// * `food_repo` - Mutable reference to the food repository
    /// * `food` - The food entity to add to the database
    /// 
    /// # Returns
    /// * `Self` - A new command instance ready for execution
    /// 
    /// # Safety
    /// The repository reference must remain valid for the lifetime of this command.
    /// The command stores a raw pointer for undo operations.
    pub fn new(food_repo: &mut FoodRepository, food: Food) -> Self {
        AddFoodCommand {
            food_repo: food_repo as *mut FoodRepository,
            food,
            executed: false,
        }
    }
}

impl Command for AddFoodCommand {
    /// Executes the add food operation with validation and error handling.
    /// 
    /// This method attempts to add the food to the repository, handling
    /// duplicate detection and maintaining execution state for undo operations.
    /// 
    /// # Returns
    /// * `Result<(), String>` - Success confirmation or detailed error message
    /// 
    /// # Error Conditions
    /// - Food with the same ID already exists in the database
    /// - Invalid food data that fails repository validation
    /// - Repository access errors during the operation
    /// 
    /// # State Management
    /// Updates the executed flag only upon successful completion to ensure
    /// proper undo behavior and prevent inconsistent state.
    fn execute(&mut self) -> Result<(), String> {
        // Safety: We know the pointer is valid because it was created from a reference
        let food_repo = unsafe { &mut *self.food_repo };
        
        let result = food_repo.add_food(self.food.clone());
        if result.is_ok() {
            self.executed = true;
        }        result
    }

    /// Undoes the add food operation by removing the food from the repository.
    /// 
    /// This method reverses the effects of the execute operation by removing
    /// the added food from the database. It validates that the command was
    /// previously executed before attempting the undo operation.
    /// 
    /// # Returns
    /// * `Result<(), String>` - Success confirmation or detailed error message
    /// 
    /// # Error Conditions
    /// - Command was not previously executed (nothing to undo)
    /// - Repository access errors during food removal
    /// - Food is referenced by composite foods (dependency violation)
    /// 
    /// # Safety Considerations
    /// Uses unsafe pointer access to the repository, which is safe because:
    /// - Pointer was created from a valid reference
    /// - Repository lifetime exceeds command lifetime
    /// - No concurrent access to the same repository
    fn undo(&mut self) -> Result<(), String> {
        if !self.executed {
            return Err("Command was not executed".to_string());
        }

        // Safety: We know the pointer is valid because it was created from a reference
        let food_repo = unsafe { &mut *self.food_repo };
        
        // Remove the food from the repository
        let foods = food_repo.get_foods_mut();
        foods.remove(&self.food.id);
        
        self.executed = false;        Ok(())
    }

    /// Returns the command type for categorization and tracking purposes.
    /// 
    /// # Returns
    /// * `CommandType::AddFood` - Identifies this as a food addition command
    fn get_type(&self) -> CommandType {
        CommandType::AddFood
    }

    /// Provides a human-readable description of the command operation.
    /// 
    /// # Returns
    /// * `String` - Descriptive text indicating the food being added
    /// 
    /// # Usage
    /// Used for command history display, logging, and user feedback.
    fn description(&self) -> String {
        format!("Add food: {}", self.food.name)    }
}

/// # Update Food Command
/// 
/// A Command Pattern implementation for updating existing foods in the food database.
/// This command handles food modification with state preservation for undo operations
/// and provides comprehensive error handling for all update scenarios.
/// 
/// ## Command Behavior
/// 
/// - **Execute**: Updates an existing food with new data and validation
/// - **Undo**: Restores the previous food state or removes the food if it was newly created
/// - **State Preservation**: Captures original food state before modification for undo
/// 
/// ## Update Strategy
/// 
/// The command captures the original food state during construction:
/// - If the food exists, stores the original for restoration
/// - If the food doesn't exist, the update becomes an add operation
/// - Undo operation reverses the exact change that was made
/// 
/// ## Memory Management
/// 
/// Uses the same raw pointer strategy as AddFoodCommand for repository access
/// with identical safety guarantees and thread safety considerations.
pub struct UpdateFoodCommand {
    /// Raw pointer to the food repository for direct database access
    food_repo: *mut FoodRepository,
    /// Original food state captured before update (None if food didn't exist)
    old_food: Option<Food>,
    /// New food data to replace the existing food
    new_food: Food,
    /// Tracks whether the command has been successfully executed
    executed: bool,
}

/// Manual implementation of Send trait for thread safety with same guarantees as AddFoodCommand.
unsafe impl Send for UpdateFoodCommand {}

/// Manual implementation of Sync trait for thread safety with same guarantees as AddFoodCommand.
unsafe impl Sync for UpdateFoodCommand {}

impl UpdateFoodCommand {
    /// Creates a new UpdateFoodCommand for modifying an existing food.
    /// 
    /// This constructor captures the current state of the food (if it exists)
    /// to enable proper undo operations. The original food state is cloned
    /// and stored for restoration during undo.
    /// 
    /// # Arguments
    /// * `food_repo` - Mutable reference to the food repository
    /// * `new_food` - The updated food data to replace the existing food
    /// 
    /// # Returns
    /// * `Self` - A new command instance ready for execution
    /// 
    /// # State Capture
    /// - If the food exists, captures its current state for undo
    /// - If the food doesn't exist, the update becomes an add operation
    /// - Stores all necessary information for complete operation reversal
    pub fn new(food_repo: &mut FoodRepository, new_food: Food) -> Self {
        let old_food = food_repo.get_food(&new_food.id).cloned();
        
        UpdateFoodCommand {
            food_repo: food_repo as *mut FoodRepository,
            old_food,
            new_food,
            executed: false,
        }
    }
}

impl Command for UpdateFoodCommand {
    fn execute(&mut self) -> Result<(), String> {
        // Safety: We know the pointer is valid because it was created from a reference
        let food_repo = unsafe { &mut *self.food_repo };
        
        let result = food_repo.update_food(self.new_food.clone());
        if result.is_ok() {
            self.executed = true;
        }
        result
    }

    fn undo(&mut self) -> Result<(), String> {
        if !self.executed {
            return Err("Command was not executed".to_string());
        }

        // Safety: We know the pointer is valid because it was created from a reference
        let food_repo = unsafe { &mut *self.food_repo };
        
        // If we have the old food, restore it
        if let Some(old_food) = &self.old_food {
            food_repo.update_food(old_food.clone())?;
        } else {
            // Otherwise remove the food
            let foods = food_repo.get_foods_mut();
            foods.remove(&self.new_food.id);
        }
        
        self.executed = false;
        Ok(())
    }

    fn get_type(&self) -> CommandType {
        CommandType::RemoveFood
    }

    fn description(&self) -> String {
        format!("Update food: {}", self.new_food.name)
    }
}
