// src/commands/food_commands.rs
use crate::models::command::{Command, CommandType};
use crate::models::food::Food;
use crate::repositories::food_repository::FoodRepository;

pub struct AddFoodCommand {
    food_repo: *mut FoodRepository,
    food: Food,
    executed: bool,
}

// Note: We need to implement Send + Sync manually because of the raw pointer
// This is safe because we ensure the pointer is valid when we use it
unsafe impl Send for AddFoodCommand {}
unsafe impl Sync for AddFoodCommand {}

impl AddFoodCommand {
    pub fn new(food_repo: &mut FoodRepository, food: Food) -> Self {
        AddFoodCommand {
            food_repo: food_repo as *mut FoodRepository,
            food,
            executed: false,
        }
    }
}

impl Command for AddFoodCommand {
    fn execute(&mut self) -> Result<(), String> {
        // Safety: We know the pointer is valid because it was created from a reference
        let food_repo = unsafe { &mut *self.food_repo };
        
        let result = food_repo.add_food(self.food.clone());
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
        
        // Remove the food from the repository
        let foods = food_repo.get_foods_mut();
        foods.remove(&self.food.id);
        
        self.executed = false;
        Ok(())
    }

    fn get_type(&self) -> CommandType {
        CommandType::AddFood
    }

    fn description(&self) -> String {
        format!("Add food: {}", self.food.name)
    }
}

pub struct UpdateFoodCommand {
    food_repo: *mut FoodRepository,
    old_food: Option<Food>,
    new_food: Food,
    executed: bool,
}

// Note: We need to implement Send + Sync manually because of the raw pointer
unsafe impl Send for UpdateFoodCommand {}
unsafe impl Sync for UpdateFoodCommand {}

impl UpdateFoodCommand {
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
