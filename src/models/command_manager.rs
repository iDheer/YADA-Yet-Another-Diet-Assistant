// src/models/command_manager.rs
use crate::models::command::Command;

pub struct CommandManager {
    undo_stack: Vec<Box<dyn Command>>,
    max_stack_size: usize,
}

impl CommandManager {
    pub fn new(max_stack_size: usize) -> Self {
        CommandManager {
            undo_stack: Vec::new(),
            max_stack_size,
        }
    }
    
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
    
    pub fn undo_last_command(&mut self) -> Result<(), String> {
        if let Some(mut command) = self.undo_stack.pop() {
            command.undo()
        } else {
            Err("No command to undo".to_string())
        }
    }
    
    pub fn get_undo_stack_size(&self) -> usize {
        self.undo_stack.len()
    }
    
    pub fn has_commands_to_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    
    pub fn get_command_history(&self) -> Vec<String> {
        self.undo_stack
            .iter()
            .map(|cmd| cmd.description())
            .collect()
    }
}
