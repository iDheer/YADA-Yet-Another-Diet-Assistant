// src/main.rs
// YADA (Yet Another Diet Assistant) - Main Application Entry Point
//
// This is the main module for the YADA diet tracking application.
// The application implements multiple design patterns including:
// - Command Pattern for undo functionality
// - Repository Pattern for data access
// - Strategy Pattern for calorie calculations
// - Factory Pattern for extensible component creation
// - Composite Pattern for complex food items

// Module declarations - each module handles a specific aspect of the application
mod models;       // Core data structures and business logic
mod repositories; // Data access layer for persistent storage
mod commands;     // Command pattern implementation for undo functionality
mod strategies;   // Strategy pattern for different calorie calculation methods
mod factories;    // Factory pattern for creating extensible components

// Standard library imports for I/O operations and data structures
use std::io::{self, Write};
use std::collections::HashSet;
use chrono::{Local, NaiveDate}; // Date/time handling

// Import core models for the application
use models::command_manager::CommandManager;
use models::profile::{Gender, ActivityLevel, UserProfile, DailyProfile};
use models::food::Food;

// Import repository pattern implementations for data persistence
use repositories::food_repository::FoodRepository;
use repositories::log_repository::LogRepository;
use repositories::profile_repository::ProfileRepository;

// Import command pattern implementations for undo functionality
use commands::food_commands::AddFoodCommand;
use commands::log_commands::{AddLogEntryCommand, RemoveLogEntryCommand};
use commands::profile_commands::{UpdateUserProfileCommand, UpdateDailyProfileCommand};

// Import strategy pattern for calorie calculations
use strategies::calorie_calculator::CalorieCalculatorFactory;

// Import factory pattern for extensible food sources
use factories::food_source_factory::FoodSourceFactory;

// Enumeration representing all possible menu options in the application
// This provides a type-safe way to handle user menu selections
enum MenuOption {
    ManageFood,   // Add or create new foods (basic/composite)
    ViewFood,     // Display all foods in the database
    LogFood,      // Record food consumption for the current date
    ViewLog,      // View and manage food consumption logs (with delete functionality)
    ManageProfile, // Update user profile information
    ViewStats,    // Display nutritional statistics and calorie calculations
    ChangeDate,   // Change the current working date for the application
    SaveData,     // Manually save all data to persistent storage
    Undo,         // Undo the last executed command
    Exit,         // Exit the application with automatic data saving
}

// Main application structure containing all repositories and managers
// This implements the Facade pattern by providing a unified interface to the complex subsystem
struct App {
    // Repository pattern implementations for data persistence
    food_repo: FoodRepository,           // Manages the food database
    log_repo: LogRepository,             // Manages daily food consumption logs
    profile_repo: ProfileRepository,     // Manages user profile data
    
    // Command pattern for undo functionality
    command_manager: CommandManager,     // Tracks and manages command history
    
    // Factory patterns for extensible architecture
    calculator_factory: CalorieCalculatorFactory, // Creates calorie calculation strategies
    food_source_factory: FoodSourceFactory,       // Creates food source implementations (extensible)
    
    // Application state
    current_date: NaiveDate,            // Current working date for logging operations
}

impl App {
    /// Creates a new instance of the YADA application
    /// Initializes all repositories, managers, and factories
    /// Seeds the food database with initial foods if empty
    /// Returns: Result containing the App instance or an IO error
    fn new() -> Result<Self, io::Error> {
        // Initialize repositories for data persistence
        let food_repo = FoodRepository::new("foods.txt")?;
        let log_repo = LogRepository::new("logs.txt")?;
        let profile_repo = ProfileRepository::new("profile.txt")?;
        
        // Initialize command manager with a capacity of 100 commands for undo functionality
        let command_manager = CommandManager::new(100);
        
        // Initialize factory patterns for extensible architecture
        let calculator_factory = CalorieCalculatorFactory::new();
        let food_source_factory = FoodSourceFactory::new();
        
        // Set current date as the working date for the application
        let current_date = Local::now().date_naive();
        
        let mut app = App {
            food_repo,
            log_repo,
            profile_repo,
            command_manager,
            calculator_factory,
            food_source_factory,
            current_date,
        };
        
        // Seed the database with initial foods if it's empty (first-time setup)
        if app.food_repo.get_all_foods().is_empty() {
            app.seed_initial_foods();
        }
        
        Ok(app)
    }
      /// Seeds the food database with a comprehensive set of basic and composite foods
    /// This method is called during first-time application setup when the food database is empty
    /// Creates 24 basic foods across different categories and 2 composite foods as examples
    fn seed_initial_foods(&mut self) {
        println!("Initializing food database with basic foods...");
        
        // Helper closure for adding basic foods with error handling
        // Parameters: id, name, keywords (comma-separated), calories per serving
        let mut add_basic_food = |id: &str, name: &str, keywords: &str, calories: f64| {
            let kw_set: HashSet<String> = keywords.split(',')
                .map(|s| s.trim().to_lowercase().to_string())
                .collect();
            let food = Food::new_basic(id.to_string(), name.to_string(), kw_set, calories);
            self.food_repo.add_food(food).ok(); // Ignore errors during seeding
        };
        
        // === DAIRY PRODUCTS ===
        add_basic_food("milk_whole", "Whole Milk (1 cup)", "milk,dairy,drink", 150.0);
        add_basic_food("milk_skim", "Skim Milk (1 cup)", "milk,dairy,drink,skim", 90.0);
        add_basic_food("cheese_cheddar", "Cheddar Cheese (1 oz)", "cheese,dairy,cheddar", 110.0);
        add_basic_food("yogurt_plain", "Plain Yogurt (1 cup)", "yogurt,dairy", 120.0);
        
        // === MEAT & PROTEIN ===
        add_basic_food("chicken_breast", "Chicken Breast (4 oz)", "chicken,meat,protein", 170.0);
        add_basic_food("beef_ground", "Ground Beef 85% (4 oz)", "beef,meat,protein", 240.0);
        add_basic_food("eggs", "Eggs (1 large)", "eggs,protein", 70.0);
        add_basic_food("tuna", "Tuna (1 can)", "tuna,fish,protein", 180.0);
        
        // === FRUITS ===
        add_basic_food("apple", "Apple (medium)", "apple,fruit", 95.0);
        add_basic_food("banana", "Banana (medium)", "banana,fruit", 105.0);
        add_basic_food("orange", "Orange (medium)", "orange,fruit,citrus", 65.0);
        add_basic_food("strawberries", "Strawberries (1 cup)", "strawberry,fruit,berries", 50.0);
        
        // === VEGETABLES ===
        add_basic_food("broccoli", "Broccoli (1 cup)", "broccoli,vegetable,veggie", 55.0);
        add_basic_food("carrot", "Carrot (medium)", "carrot,vegetable,veggie", 25.0);
        add_basic_food("spinach", "Spinach (1 cup)", "spinach,vegetable,veggie,leafy", 7.0);
        add_basic_food("potato", "Potato (medium)", "potato,vegetable,starchy", 110.0);
        
        // === GRAINS & STARCHES ===
        add_basic_food("bread_wheat", "Wheat Bread (1 slice)", "bread,grain,wheat", 80.0);
        add_basic_food("rice_white", "White Rice (1 cup cooked)", "rice,grain,white", 200.0);
        add_basic_food("pasta", "Pasta (1 cup cooked)", "pasta,grain", 220.0);
        add_basic_food("oatmeal", "Oatmeal (1 cup cooked)", "oatmeal,grain,breakfast", 160.0);
        
        // === OTHER FOODS ===
        add_basic_food("peanut_butter", "Peanut Butter (2 tbsp)", "peanut,butter,spread", 190.0);
        add_basic_food("jelly", "Grape Jelly (1 tbsp)", "jelly,grape,spread", 50.0);
        add_basic_food("olive_oil", "Olive Oil (1 tbsp)", "oil,fat", 120.0);
        add_basic_food("soda", "Soda (12 oz can)", "soda,drink,sugar", 150.0);        
        // === COMPOSITE FOODS DEMONSTRATION ===
        // Create example composite foods to show the Composite pattern implementation
        
        // First composite food: Peanut Butter Sandwich (bread + peanut butter)
        let mut pb_sandwich = Food::new_composite(
            "pb_sandwich".to_string(),
            "Peanut Butter Sandwich".to_string(),
            ["sandwich", "peanut butter", "lunch"].iter().map(|s| s.to_string()).collect(),
            vec![("bread_wheat".to_string(), 2.0), ("peanut_butter".to_string(), 1.0)]
        );
        
        // Calculate total calories by summing component calories * servings
        let mut total_calories = 0.0;
        for (comp_id, servings) in &pb_sandwich.components {
            if let Some(component) = self.food_repo.get_food(comp_id) {
                total_calories += component.calories_per_serving * servings;
            }
        }
        pb_sandwich.calories_per_serving = total_calories;
        self.food_repo.add_food(pb_sandwich).ok();
        
        // Second composite food: PB&J Sandwich (extends pb_sandwich with jelly)
        // This demonstrates composites can contain other composites
        let mut pbj_sandwich = Food::new_composite(
            "pbj_sandwich".to_string(),
            "PB&J Sandwich".to_string(),
            ["sandwich", "peanut butter", "jelly", "lunch"].iter().map(|s| s.to_string()).collect(),
            vec![("pb_sandwich".to_string(), 1.0), ("jelly".to_string(), 1.0)]
        );
        
        // Calculate calories for this composite food
        let mut total_calories = 0.0;
        for (comp_id, servings) in &pbj_sandwich.components {
            if let Some(component) = self.food_repo.get_food(comp_id) {
                total_calories += component.calories_per_serving * servings;
            }
        }
        pbj_sandwich.calories_per_serving = total_calories;
        self.food_repo.add_food(pbj_sandwich).ok();
        
        println!("Food database initialized with {} basic foods and 2 composite foods.", 24);
        
        // Persist the seeded database to the file system
        if let Err(e) = self.food_repo.save() {
            println!("Warning: Failed to save seeded food database: {}", e);
        }
    }
      /// Main application loop that handles user interaction and menu navigation
    /// 
    /// This method implements the main event loop of the application:
    /// 1. Welcomes the user and ensures a profile exists (creates one if needed)
    /// 2. Displays the main menu and processes user choices
    /// 3. Delegates to appropriate handler methods based on user selection
    /// 4. Automatically saves data before exiting
    /// 
    /// The loop continues until the user chooses to exit, ensuring persistent
    /// application state and clean shutdown with data preservation.
    fn run(&mut self) {
        println!("Welcome to YADA (Yet Another Diet Assistant)!");
        
        // Check if we have a user profile - required for calorie calculations
        if self.profile_repo.get_profile().is_none() {
            println!("No user profile found. Let's create one!");
            self.create_initial_profile();
        }
        
        // Main application event loop - continues until user exits
        loop {
            match self.show_main_menu() {
                MenuOption::ManageFood => self.manage_foods(),        // Add/create foods
                MenuOption::ViewFood => self.view_foods(),            // Display food database
                MenuOption::LogFood => self.log_food(),               // Record consumption
                MenuOption::ViewLog => self.view_log(),               // View/manage logs
                MenuOption::ManageProfile => self.manage_profile(),   // Update user profile
                MenuOption::ViewStats => self.view_stats(),           // Show statistics
                MenuOption::ChangeDate => self.change_date(),         // Change working date
                MenuOption::SaveData => self.save_data(),             // Manual data save
                MenuOption::Undo => self.undo_last_command(),         // Undo last action
                MenuOption::Exit => {
                    self.save_data();  // Automatic save on exit
                    println!("Goodbye!");
                    break;
                }
            }
        }
    }
      /// Displays the main menu and captures user input for menu selection
    /// 
    /// This method provides the primary user interface for the application:
    /// 1. Shows the current working date for context
    /// 2. Lists all available menu options with numbered choices
    /// 3. Validates user input and returns the corresponding MenuOption
    /// 4. Loops until a valid choice is entered
    /// 
    /// The menu includes options for food management, logging, profile management,
    /// statistics viewing, date changes, data persistence, and undo functionality.
    /// 
    /// Returns: MenuOption enum representing the user's choice
    fn show_main_menu(&self) -> MenuOption {
        println!("\n------ YADA Main Menu ------");
        println!("Current date: {}", self.current_date.format("%Y-%m-%d"));
        println!("1. Manage Foods");
        println!("2. View Foods");
        println!("3. Log Food Consumption");
        println!("4. View Food Log");
        println!("5. Manage Profile");
        println!("6. View Statistics");
        println!("7. Change Current Date");  // Added new menu option
        println!("8. Save Data");
        println!("9. Undo Last Action");
        println!("10. Exit");
        println!("----------------------------");
        
        // Input validation loop - continues until valid choice is entered
        loop {
            print!("Enter your choice (1-10): ");  // Updated range
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim().parse::<u32>() {
                Ok(1) => return MenuOption::ManageFood,
                Ok(2) => return MenuOption::ViewFood,
                Ok(3) => return MenuOption::LogFood,
                Ok(4) => return MenuOption::ViewLog,
                Ok(5) => return MenuOption::ManageProfile,
                Ok(6) => return MenuOption::ViewStats,
                Ok(7) => return MenuOption::ChangeDate, // Added new option
                Ok(8) => return MenuOption::SaveData,
                Ok(9) => return MenuOption::Undo,
                Ok(10) => return MenuOption::Exit,
                _ => println!("Invalid choice. Please enter a number between 1 and 10."),
            }
        }
    }
      /// Allows the user to change the current working date for the application
    /// 
    /// This method provides date management functionality:
    /// 1. Shows the current working date for reference
    /// 2. Accepts either a specific date (YYYY-MM-DD) or 'today' for current date
    /// 3. Validates date format and updates the application state
    /// 4. Loops until a valid date is entered
    /// 
    /// The working date affects all date-sensitive operations including:
    /// - Food logging (entries are recorded for the current date)
    /// - Log viewing (shows entries for the current date)
    /// - Statistics (calculates metrics for the current date)
    /// - Profile data (uses current date for age calculations and daily profiles)
    fn change_date(&mut self) {
        println!("\n------ Change Current Date ------");
        println!("Current date: {}", self.current_date.format("%Y-%m-%d"));
        
        // Input validation loop for date selection
        loop {
            print!("Enter new date (YYYY-MM-DD) or 'today' for current date: ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            
            if input.to_lowercase() == "today" {
                // Set to system's current date
                self.current_date = Local::now().date_naive();
                println!("Date set to today: {}", self.current_date.format("%Y-%m-%d"));
                break;
            } else {
                // Parse user-provided date with validation
                match NaiveDate::parse_from_str(&input, "%Y-%m-%d") {
                    Ok(date) => {
                        self.current_date = date;
                        println!("Date changed to: {}", self.current_date.format("%Y-%m-%d"));
                        break;
                    },
                    Err(_) => println!("Invalid date format. Please use YYYY-MM-DD."),
                }
            }
        }
    }
      /// Searches the food database based on user-provided keywords
    /// 
    /// This method implements flexible food search functionality:
    /// 1. Prompts user for comma-separated search keywords
    /// 2. Offers choice between AND search (all keywords must match) and OR search (any keyword matches)
    /// 3. Filters the food database based on the selected criteria
    /// 4. Returns a vector of food references that match the search
    /// 
    /// The search is case-insensitive and matches against the keywords stored
    /// with each food item. This enables users to quickly find foods without
    /// browsing the entire database.
    /// 
    /// Returns: Vector of Food references matching the search criteria
    fn search_foods(&self) -> Vec<&Food> {
        println!("\n------ Search Foods ------");
        
        // Get search keywords from user input
        print!("Enter search keywords (comma-separated): ");
        io::stdout().flush().unwrap();
        
        let mut keywords_str = String::new();
        io::stdin().read_line(&mut keywords_str).unwrap();
        
        // Parse and normalize keywords (convert to lowercase, remove empty strings)
        let keywords: HashSet<String> = keywords_str
            .trim()
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();
        
        // Handle case where no valid keywords were entered
        if keywords.is_empty() {
            println!("No valid keywords entered. Returning all foods.");
            return self.food_repo.get_all_foods();
        }
        
        // Determine search mode (AND vs OR)
        println!("Match all keywords or any keyword?");
        println!("1. Match ANY keyword (OR search)");
        println!("2. Match ALL keywords (AND search)");
        
        print!("Enter your choice (1-2): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let match_all = match input.trim().parse::<u32>() {
            Ok(1) => false,  // OR search
            Ok(2) => true,   // AND search
            _ => {
                println!("Invalid choice. Using ANY keyword matching.");
                false
            }
        };
        
        // Perform the search based on selected criteria
        let mut results = Vec::new();
        
        for food in self.food_repo.get_all_foods() {
            let matches = if match_all {
                // AND search - all keywords must be present in food's keywords
                keywords.iter().all(|k| food.keywords.contains(k))
            } else {
                // OR search - at least one keyword must be present
                keywords.iter().any(|k| food.keywords.contains(k))
            };
            
            if matches {
                results.push(food);
            }
        }
        
        println!("Found {} foods matching your search criteria.", results.len());
        
        results
    }
      /// Creates an initial user profile for new users
    /// 
    /// This method guides new users through the profile creation process:
    /// 1. Collects basic biographical information (gender, height, birth date)
    /// 2. Records current weight and activity level for the current date
    /// 3. Creates both a UserProfile and initial DailyProfile
    /// 4. Stores the profile in the repository for future use
    /// 
    /// The profile information is essential for:
    /// - Calorie calculation strategies (BMR/TDEE calculations)
    /// - Age-based nutritional recommendations
    /// - Activity level adjustments for calorie targets
    /// - Weight tracking over time
    /// 
    /// Input validation ensures all data is within reasonable ranges
    /// and properly formatted before creating the profile.
    fn create_initial_profile(&mut self) {
        println!("\n------ Create User Profile ------");
        
        // Collect gender information for BMR calculations
        println!("Select your gender:");
        println!("1. Male");
        println!("2. Female");
        println!("3. Other");
        
        let gender = loop {
            print!("Enter your choice (1-3): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim().parse::<u32>() {
                Ok(1) => break Gender::Male,
                Ok(2) => break Gender::Female,
                Ok(3) => break Gender::Other,
                _ => println!("Invalid choice. Please enter a number between 1 and 3."),
            }
        };
        
        // Collect height (required for BMR calculations)
        let height = loop {
            print!("Enter your height in centimeters: ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim().parse::<f64>() {
                Ok(h) if h > 0.0 => break h,
                _ => println!("Invalid height. Please enter a positive number."),
            }
        };
        
        // Collect birth date (for age calculation)
        let birth_date = loop {
            print!("Enter your birth date (YYYY-MM-DD): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match NaiveDate::parse_from_str(input.trim(), "%Y-%m-%d") {
                Ok(date) => break date,
                Err(_) => println!("Invalid date format. Please use YYYY-MM-DD."),
            }
        };
        
        // Create the basic user profile with biographical data
        let mut profile = UserProfile::new(gender, height, birth_date);
        
        // Collect current day's variable data (weight and activity level)
        let weight = loop {
            print!("Enter your current weight in kilograms: ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim().parse::<f64>() {
                Ok(w) if w > 0.0 => break w,
                _ => println!("Invalid weight. Please enter a positive number."),
            }
        };
        
        // Activity level affects TDEE calculations
        println!("Select your activity level:");
        println!("1. Sedentary (little or no exercise)");
        println!("2. Lightly active (light exercise/sports 1-3 days/week)");
        println!("3. Moderately active (moderate exercise/sports 3-5 days/week)");
        println!("4. Very active (hard exercise/sports 6-7 days a week)");
        println!("5. Extremely active (very hard exercise & physical job or training twice a day)");
        
        let activity_level = loop {
            print!("Enter your choice (1-5): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim().parse::<u32>() {
                Ok(1) => break ActivityLevel::Sedentary,
                Ok(2) => break ActivityLevel::LightlyActive,
                Ok(3) => break ActivityLevel::ModeratelyActive,
                Ok(4) => break ActivityLevel::VeryActive,
                Ok(5) => break ActivityLevel::ExtremelyActive,
                _ => println!("Invalid choice. Please enter a number between 1 and 5."),
            }
        };
        
        // Create daily profile for the current date
        let daily_profile = DailyProfile {
            date: self.current_date,
            weight,
            activity_level,
        };
        
        // Add the daily profile to the user profile
        profile.add_or_update_daily_profile(daily_profile);
        
        // Store the completed profile in the repository
        self.profile_repo.set_profile(profile);
        println!("Profile created successfully!");
    }
      /// Provides a sub-menu for food management operations
    /// 
    /// This method creates a dedicated interface for food-related operations:
    /// 1. Add Basic Food - Create simple food items with direct calorie values
    /// 2. Create Composite Food - Build complex foods from existing components
    /// 3. Return to Main Menu - Exit the food management interface
    /// 
    /// The method implements a loop that continues until the user chooses
    /// to return to the main menu, allowing multiple food operations in sequence.
    /// This design follows the single responsibility principle by grouping
    /// related food management functionality.
    fn manage_foods(&mut self) {
        loop {
            println!("\n------ Manage Foods ------");
            println!("1. Add Basic Food");
            println!("2. Create Composite Food");
            println!("3. Back to Main Menu");
            
            print!("Enter your choice (1-3): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim().parse::<u32>() {
                Ok(1) => self.add_basic_food(),      // Delegate to basic food creation
                Ok(2) => self.create_composite_food(), // Delegate to composite food creation
                Ok(3) => break,                       // Exit food management menu
                _ => println!("Invalid choice. Please enter a number between 1 and 3."),
            }
        }
    }
      /// Creates and adds a basic food item to the database using the Command pattern
    /// 
    /// This method handles the creation of simple food items with the following process:
    /// 1. Collects food identification information (ID and name)
    /// 2. Validates that the food ID is unique in the database
    /// 3. Gathers search keywords for easy food discovery
    /// 4. Records the calorie content per serving
    /// 5. Creates the food object and uses Command pattern for undo support
    /// 
    /// Input validation ensures:
    /// - Food ID uniqueness to prevent duplicates
    /// - Non-negative calorie values for nutritional accuracy
    /// - Proper keyword formatting for search functionality
    /// 
    /// Uses the Command pattern to enable undo functionality for food additions.
    fn add_basic_food(&mut self) {
        println!("\n------ Add Basic Food ------");
        
        // Collect unique food identifier
        print!("Enter food ID (no spaces): ");
        io::stdout().flush().unwrap();
        let mut id = String::new();
        io::stdin().read_line(&mut id).unwrap();
        id = id.trim().to_string();
        
        // Ensure food ID is unique to prevent conflicts
        if self.food_repo.get_food(&id).is_some() {
            println!("A food with ID '{}' already exists.", id);
            return;
        }
        
        // Collect human-readable food name
        print!("Enter food name: ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        name = name.trim().to_string();
        
        // Collect search keywords for food discovery
        print!("Enter keywords (comma-separated): ");
        io::stdout().flush().unwrap();
        let mut keywords_str = String::new();
        io::stdin().read_line(&mut keywords_str).unwrap();
        
        // Parse and normalize keywords for consistent searching
        let keywords: HashSet<String> = keywords_str
            .trim()
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();
        
        // Collect nutritional information with validation
        print!("Enter calories per serving: ");
        io::stdout().flush().unwrap();
        let mut calories_str = String::new();
        io::stdin().read_line(&mut calories_str).unwrap();
        
        let calories = match calories_str.trim().parse::<f64>() {
            Ok(c) if c >= 0.0 => c,
            _ => {
                println!("Invalid calories. Please enter a non-negative number.");
                return;
            }
        };
        
        // Create food object and add using Command pattern for undo support
        let food = Food::new_basic(id, name, keywords, calories);
        let command = Box::new(AddFoodCommand::new(&mut self.food_repo, food));
        
        match self.command_manager.execute_command(command) {
            Ok(_) => println!("Food added successfully!"),
            Err(e) => println!("Error adding food: {}", e),
        }
    }
      /// Creates a composite food item built from existing food components (Composite Pattern)
    /// 
    /// This method implements the Composite Pattern for complex food creation:
    /// 1. Collects basic food information (ID, name, keywords)
    /// 2. Allows user to specify multiple component foods with servings
    /// 3. Validates that all component foods exist in the database
    /// 4. Creates a composite food whose calories are calculated from components
    /// 5. Uses Command pattern for undo support
    /// 
    /// Composite foods enable modeling of:
    /// - Recipes (e.g., sandwich made from bread, meat, cheese)
    /// - Meals (e.g., breakfast combining multiple food items)
    /// - Complex dishes with multiple ingredients
    /// 
    /// The calorie content is automatically calculated by summing the calories
    /// of all components multiplied by their respective serving amounts.
    fn create_composite_food(&mut self) {
        println!("\n------ Create Composite Food ------");
        
        // Collect basic food identification (same as basic foods)
        print!("Enter food ID (no spaces): ");
        io::stdout().flush().unwrap();
        let mut id = String::new();
        io::stdin().read_line(&mut id).unwrap();
        id = id.trim().to_string();
        
        // Ensure uniqueness across all food types
        if self.food_repo.get_food(&id).is_some() {
            println!("A food with ID '{}' already exists.", id);
            return;
        }
        
        print!("Enter food name: ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        name = name.trim().to_string();
        
        print!("Enter keywords (comma-separated): ");
        io::stdout().flush().unwrap();
        let mut keywords_str = String::new();
        io::stdin().read_line(&mut keywords_str).unwrap();
        
        let keywords: HashSet<String> = keywords_str
            .trim()
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();
        
        // Collect component foods and their quantities
        let mut components: Vec<(String, f64)> = Vec::new();
        
        println!("Add components (enter empty food ID to finish):");
        loop {
            print!("Enter component food ID: ");
            io::stdout().flush().unwrap();
            let mut comp_id = String::new();
            io::stdin().read_line(&mut comp_id).unwrap();
            comp_id = comp_id.trim().to_string();
            
            // Empty input signals completion of component entry
            if comp_id.is_empty() {
                break;
            }
            
            // Validate that the component food exists in the database
            if self.food_repo.get_food(&comp_id).is_none() {
                println!("Food with ID '{}' doesn't exist.", comp_id);
                continue;
            }
            
            // Get the quantity of this component
            print!("Enter number of servings: ");
            io::stdout().flush().unwrap();
            let mut servings_str = String::new();
            io::stdin().read_line(&mut servings_str).unwrap();
            
            let servings = match servings_str.trim().parse::<f64>() {
                Ok(s) if s > 0.0 => s,
                _ => {
                    println!("Invalid servings. Please enter a positive number.");
                    continue;
                }
            };
            
            // Add the validated component to the list
            components.push((comp_id, servings));
        }
        
        // Ensure at least one component was added
        if components.is_empty() {
            println!("No components added. Cannot create composite food.");
            return;
        }
        
        // Create composite food using the Composite Pattern
        let food = Food::new_composite(id, name, keywords, components);
        let command = Box::new(AddFoodCommand::new(&mut self.food_repo, food));
        
        match self.command_manager.execute_command(command) {
            Ok(_) => println!("Composite food added successfully!"),
            Err(e) => println!("Error adding composite food: {}", e),
        }
    }
      /// Displays all foods in the database in a formatted table
    /// 
    /// This method provides a comprehensive view of the food database:
    /// 1. Retrieves all foods from the repository
    /// 2. Displays them in a formatted table with columns for ID, Name, Keywords, and Calories
    /// 3. Handles empty database gracefully with appropriate messaging
    /// 4. Formats keywords as comma-separated strings for readability
    /// 
    /// The tabular format makes it easy for users to:
    /// - Browse available foods before logging consumption
    /// - See nutritional information at a glance
    /// - Identify foods by their keywords for search purposes
    /// - Copy food IDs for use in logging or composite food creation
    fn view_foods(&self) {
        println!("\n------ View Foods ------");
        
        let foods = self.food_repo.get_all_foods();
        
        // Handle empty database case
        if foods.is_empty() {
            println!("No foods in database.");
            return;
        }
        
        // Display formatted table header
        println!("{:<10} {:<20} {:<30} {:<10}", "ID", "Name", "Keywords", "Calories");
        println!("{:-<75}", "");
        
        // Display each food with formatted columns
        for food in foods {
            let keywords_str = food.keywords.iter().cloned().collect::<Vec<_>>().join(", ");
            println!("{:<10} {:<20} {:<30} {:<10.1}", 
                    food.id, food.name, keywords_str, food.calories_per_serving);
        }
    }
      /// Records food consumption for the current date using the Command pattern
    /// 
    /// This method handles food logging with the following workflow:
    /// 1. Offers choice between viewing all foods or searching by keywords
    /// 2. Displays available foods in a formatted table for easy selection
    /// 3. Validates that the selected food exists in the database
    /// 4. Records the number of servings consumed
    /// 5. Uses Command pattern to enable undo functionality
    /// 
    /// The search integration allows users to quickly find foods without
    /// browsing the entire database. All logged entries are associated with
    /// the current working date, enabling day-specific tracking.
    /// 
    /// Uses AddLogEntryCommand for undo support and consistent data management.
    fn log_food(&mut self) {
        println!("\n------ Log Food Consumption ------");
        
        // Ensure food database is not empty
        let foods = self.food_repo.get_all_foods();
        if foods.is_empty() {
            println!("No foods in database. Please add foods first.");
            return;
        }
        
        // Offer food selection methods
        println!("1. Show all foods");
        println!("2. Search foods by keyword");
        
        print!("Enter your choice (1-2): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        // Get foods based on user's selection method
        let selected_foods = match input.trim().parse::<u32>() {
            Ok(1) => self.food_repo.get_all_foods(),  // Show all foods
            Ok(2) => self.search_foods(),             // Use search functionality
            _ => {
                println!("Invalid choice. Showing all foods.");
                self.food_repo.get_all_foods()
            }
        };
        
        // Ensure search/selection returned results
        if selected_foods.is_empty() {
            println!("No foods found.");
            return;
        }
        
        // Display available foods for selection
        println!("\nAvailable foods:");
        println!("{:<10} {:<20} {:<10}", "ID", "Name", "Calories");
        println!("{:-<45}", "");
        
        for food in &selected_foods {
            println!("{:<10} {:<20} {:<10.1}", 
                    food.id, food.name, food.calories_per_serving);
        }
        
        // Get user's food selection
        print!("\nEnter food ID: ");
        io::stdout().flush().unwrap();
        let mut food_id = String::new();
        io::stdin().read_line(&mut food_id).unwrap();
        food_id = food_id.trim().to_string();
        
        // Validate that the selected food exists
        if self.food_repo.get_food(&food_id).is_none() {
            println!("Food with ID '{}' doesn't exist.", food_id);
            return;
        }
        
        // Get the number of servings consumed
        print!("Enter number of servings: ");
        io::stdout().flush().unwrap();
        let mut servings_str = String::new();
        io::stdin().read_line(&mut servings_str).unwrap();
        
        let servings = match servings_str.trim().parse::<f64>() {
            Ok(s) if s > 0.0 => s,
            _ => {
                println!("Invalid servings. Please enter a positive number.");
                return;
            }
        };
        
        // Create and execute log entry command for undo support
        let command = Box::new(AddLogEntryCommand::new(
            &mut self.log_repo,
            self.current_date,
            food_id,
            servings
        ));
        
        match self.command_manager.execute_command(command) {
            Ok(_) => println!("Food logged successfully!"),
            Err(e) => println!("Error logging food: {}", e),
        }
    }
    /// Displays the food log for the current date with interactive management options
    /// 
    /// This method provides a comprehensive view of daily food consumption with:
    /// 1. Formatted display of all logged food entries for the current date
    /// 2. Calculation of total calories consumed vs target calories
    /// 3. Interactive menu for deleting entries (edit functionality)
    /// 4. Real-time display updates after modifications
    /// 
    /// Display includes:
    /// - Food ID, name, servings, and calories for each entry
    /// - Total calories consumed for the day
    /// - Target calories based on user profile and calculation method
    /// - Calorie difference (surplus/deficit) for diet tracking
    /// 
    /// The method integrates with the Repository pattern to access food and log data,
    /// and the Strategy pattern for calorie calculations based on user preferences.
    fn view_log(&mut self) {
        loop {
            println!("\n------ View Food Log ------");
            
            // Get log for current date
            if let Some(log) = self.log_repo.get_log(self.current_date) {
                if log.entries.is_empty() {
                    println!("No food entries for {}", self.current_date.format("%Y-%m-%d"));
                    return;
                }
                
                println!("Food log for {}", self.current_date.format("%Y-%m-%d"));
                println!("{:<5} {:<10} {:<20} {:<10} {:<10}", "#", "Food ID", "Name", "Servings", "Calories");
                println!("{:-<60}", "");
                
                let mut total_calories = 0.0;
                
                for (i, entry) in log.entries.iter().enumerate() {
                    let food_name = self.food_repo.get_food(&entry.food_id)
                        .map_or("Unknown".to_string(), |f| f.name.clone());
                    
                    let calories = self.food_repo.get_food(&entry.food_id)
                        .map_or(0.0, |f| f.calories_per_serving * entry.servings);
                    
                    println!("{:<5} {:<10} {:<20} {:<10.1} {:<10.1}", 
                            i+1, entry.food_id, food_name, entry.servings, calories);
                    
                    total_calories += calories;
                }
                
                println!("{:-<60}", "");
                println!("Total calories: {:.1}", total_calories);
                
                // If we have a profile, show target calories
                if let Some(profile) = self.profile_repo.get_profile() {
                    let calculator = self.calculator_factory.get_calculator(&profile.calculation_method)
                        .unwrap_or_else(|| self.calculator_factory.get_calculator("harris_benedict").unwrap());
                    
                    let target_calories = calculator.calculate_target_calories(profile, self.current_date);
                    
                    println!("Target calories: {:.1}", target_calories);
                    println!("Difference: {:.1}", total_calories - target_calories);
                }
                
                // Show menu options
                println!("\nOptions:");
                println!("1. Delete a food entry");
                println!("2. Back to main menu");
                
                print!("Enter your choice (1-2): ");
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                
                match input.trim().parse::<u32>() {
                    Ok(1) => {
                        self.delete_log_entry();
                        // Continue the loop to refresh the display
                    },
                    Ok(2) => break,
                    _ => {
                        println!("Invalid choice. Please enter 1 or 2.");
                        continue;
                    }
                }
            } else {
                println!("No food entries for {}", self.current_date.format("%Y-%m-%d"));
                break;
            }
        }
    }
    
    /// Provides a comprehensive interface for user profile management
    /// 
    /// This method creates a centralized profile management hub that:
    /// 1. Displays current profile information in a formatted view
    /// 2. Shows both basic profile data (gender, height, birth date, age)
    /// 3. Displays current daily data (weight, activity level) for the active date
    /// 4. Shows the current calorie calculation method in use
    /// 5. Provides navigation to specific profile update operations
    /// 
    /// Profile management options:
    /// - Update Basic Profile: Modify static information (gender, height, birth date)
    /// - Update Today's Data: Modify current weight and activity level
    /// - Change Calculation Method: Switch between different TDEE calculation strategies
    /// 
    /// The method integrates with the Repository pattern for profile data access
    /// and provides a user-friendly interface for profile modifications while
    /// maintaining separation of concerns for different types of profile updates.
    fn manage_profile(&mut self) {
        loop {
            println!("\n------ Manage Profile ------");
            
            if let Some(profile) = self.profile_repo.get_profile() {
                println!("Current Profile:");
                println!("Gender: {:?}", profile.gender);
                println!("Height: {:.1} cm", profile.height);
                println!("Birth Date: {}", profile.birth_date.format("%Y-%m-%d"));
                println!("Age: {} years", profile.age(self.current_date));
                
                if let Some(daily) = profile.get_daily_profile(self.current_date) {
                    println!("Current Weight: {:.1} kg", daily.weight);
                    println!("Activity Level: {:?}", daily.activity_level);
                }
                
                println!("Calculation Method: {}", profile.calculation_method);
            } else {
                println!("No profile exists!");
            }
            
            println!("\n1. Update Basic Profile");
            println!("2. Update Today's Data");
            println!("3. Change Calculation Method");
            println!("4. Back to Main Menu");
            
            print!("Enter your choice (1-4): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim().parse::<u32>() {
                Ok(1) => self.update_basic_profile(),
                Ok(2) => self.update_daily_profile(),
                Ok(3) => self.change_calculation_method(),
                Ok(4) => break,
                _ => println!("Invalid choice. Please enter a number between 1 and 4."),
            }
        }
    }
    
    /// Updates the static components of a user profile (gender, height, birth date)
    /// 
    /// This method handles modification of user profile information that typically
    /// remains constant over time:
    /// 1. Gender selection with current value display and keep-current option
    /// 2. Height modification with validation for reasonable values (>0)
    /// 3. Birth date updates with proper date parsing and validation
    /// 4. Command pattern integration for undo functionality
    /// 
    /// User experience features:
    /// - Shows current values for all fields before changes
    /// - Provides "keep current" options to avoid accidental modifications
    /// - Input validation prevents invalid data entry
    /// - Clear feedback on successful updates
    /// 
    /// Uses UpdateBasicProfileCommand to maintain consistency with the
    /// application's command-based architecture, enabling undo functionality
    /// for profile modifications while preserving data integrity.
    fn update_basic_profile(&mut self) {
        println!("\n------ Update Basic Profile ------");
        
        let current_profile = match self.profile_repo.get_profile() {
            Some(p) => p.clone(),
            None => {
                println!("No profile exists! Creating a new one.");
                self.create_initial_profile();
                return;
            }
        };
        
        // Gender
        println!("Select your gender (current: {:?}):", current_profile.gender);
        println!("1. Male");
        println!("2. Female");
        println!("3. Other");
        println!("4. Keep current");
        
        let gender = loop {
            print!("Enter your choice (1-4): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim().parse::<u32>() {
                Ok(1) => break Gender::Male,
                Ok(2) => break Gender::Female,
                Ok(3) => break Gender::Other,
                Ok(4) => break current_profile.gender.clone(),
                _ => println!("Invalid choice. Please enter a number between 1 and 4."),
            }
        };
        
        // Height
        println!("Current height: {:.1} cm", current_profile.height);
        print!("Enter your height in centimeters (or leave blank to keep current): ");
        io::stdout().flush().unwrap();
        
        let mut height_str = String::new();
        io::stdin().read_line(&mut height_str).unwrap();
        height_str = height_str.trim().to_string();
        
        let height = if height_str.is_empty() {
            current_profile.height
        } else {
            match height_str.parse::<f64>() {
                Ok(h) if h > 0.0 => h,
                _ => {
                    println!("Invalid height. Keeping current height.");
                    current_profile.height
                }
            }
        };
        
        // Birth date
        println!("Current birth date: {}", current_profile.birth_date.format("%Y-%m-%d"));
        print!("Enter your birth date (YYYY-MM-DD) (or leave blank to keep current): ");
        io::stdout().flush().unwrap();
        
        let mut date_str = String::new();
        io::stdin().read_line(&mut date_str).unwrap();
        date_str = date_str.trim().to_string();
        
        let birth_date = if date_str.is_empty() {
            current_profile.birth_date
        } else {
            match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => {
                    println!("Invalid date format. Keeping current birth date.");
                    current_profile.birth_date
                }
            }
        };
        
        // Create updated profile
        let mut new_profile = UserProfile::new(gender, height, birth_date);
        
        // Copy over daily profiles and calculation method
        new_profile.calculation_method = current_profile.calculation_method;
        new_profile.daily_profiles = current_profile.daily_profiles.clone();
        
        // Update using command pattern
        let command = Box::new(UpdateUserProfileCommand::new(
            &mut self.profile_repo,
            new_profile
        ));
        
        match self.command_manager.execute_command(command) {
            Ok(_) => println!("Profile updated successfully!"),
            Err(e) => println!("Error updating profile: {}", e),
        }
    }
    
    /// Updates daily profile information (weight and activity level) for the current date
    /// 
    /// This method manages date-specific profile data that can vary day by day:
    /// 1. Current weight input with validation for positive values
    /// 2. Activity level selection from predefined categories
    /// 3. Creates or updates daily profile for the current application date
    /// 4. Command pattern integration for undo functionality
    /// 
    /// Daily profile categories:
    /// - Weight: Allows tracking of weight changes over time
    /// - Activity Level: Sedentary, Lightly Active, Moderately Active, Very Active, Extremely Active
    /// 
    /// This enables accurate TDEE calculations that account for daily variations
    /// in weight and activity, providing more precise calorie targets for
    /// effective diet management. Uses UpdateDailyProfileCommand to maintain
    /// consistency with the application's command-based architecture.
    fn update_daily_profile(&mut self) {
        println!("\n------ Update Today's Data ------");
        
        if self.profile_repo.get_profile().is_none() {
            println!("No profile exists! Please create a profile first.");
            return;
        }
        
        // Get current daily profile if it exists
        let current_daily = self.profile_repo
            .get_profile()
            .and_then(|p| p.get_daily_profile(self.current_date).cloned());
        
        // Weight
        let current_weight = current_daily.as_ref().map_or(0.0, |d| d.weight);
        println!("Current weight: {:.1} kg", current_weight);
        
        print!("Enter your weight in kilograms: ");
        io::stdout().flush().unwrap();
        
        let mut weight_str = String::new();
        io::stdin().read_line(&mut weight_str).unwrap();
        
        let weight = match weight_str.trim().parse::<f64>() {
            Ok(w) if w > 0.0 => w,
            _ => {
                println!("Invalid weight. Please enter a positive number.");
                return;
            }
        };
        
        // Activity level
        println!("Select your activity level:");
        println!("1. Sedentary (little or no exercise)");
        println!("2. Lightly active (light exercise/sports 1-3 days/week)");
        println!("3. Moderately active (moderate exercise/sports 3-5 days/week)");
        println!("4. Very active (hard exercise/sports 6-7 days a week)");
        println!("5. Extremely active (very hard exercise & physical job or training twice a day)");
        
        let activity_level = loop {
            print!("Enter your choice (1-5): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim().parse::<u32>() {
                Ok(1) => break ActivityLevel::Sedentary,
                Ok(2) => break ActivityLevel::LightlyActive,
                Ok(3) => break ActivityLevel::ModeratelyActive,
                Ok(4) => break ActivityLevel::VeryActive,
                Ok(5) => break ActivityLevel::ExtremelyActive,
                _ => println!("Invalid choice. Please enter a number between 1 and 5."),
            }
        };
        
        // Create daily profile
        let daily_profile = DailyProfile {
            date: self.current_date,
            weight,
            activity_level,
        };
        
        // Update using command pattern
        let command = Box::new(UpdateDailyProfileCommand::new(
            &mut self.profile_repo,
            daily_profile
        ));
        
        match self.command_manager.execute_command(command) {
            Ok(_) => println!("Daily profile updated successfully!"),
            Err(e) => println!("Error updating daily profile: {}", e),
        }
    }
    
    /// Changes the calorie calculation method used for TDEE computations (Strategy Pattern)
    /// 
    /// This method implements the Strategy Pattern by allowing users to switch between
    /// different Total Daily Energy Expenditure (TDEE) calculation algorithms:
    /// 1. Harris-Benedict Formula: Traditional BMR calculation method
    /// 2. Mifflin-St Jeor Formula: More modern and often more accurate
    /// 3. Future extensibility for additional calculation strategies
    /// 
    /// Strategy Pattern implementation:
    /// - Factory creates appropriate calculator instances
    /// - User can switch strategies at runtime
    /// - Calculations adapt automatically to selected method
    /// - Consistent interface regardless of underlying algorithm
    /// 
    /// This flexibility allows users to choose the calculation method that works
    /// best for their body type and goals, improving the accuracy of calorie
    /// targets and overall diet management effectiveness.
    fn change_calculation_method(&mut self) {
        println!("\n------ Change Calculation Method ------");
        
        let profile = match self.profile_repo.get_profile_mut() {
            Some(p) => p,
            None => {
                println!("No profile exists! Please create a profile first.");
                return;
            }
        };
        
        println!("Available calculation methods:");
        for (i, method) in self.calculator_factory.get_all_calculators().iter().enumerate() {
            let calculator = self.calculator_factory.get_calculator(method).unwrap();
            println!("{}. {} - {}", i+1, calculator.name(), calculator.description());
        }
        
        println!("Current method: {}", profile.calculation_method);
        
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let index = match input.trim().parse::<usize>() {
            Ok(i) if i > 0 && i <= self.calculator_factory.get_all_calculators().len() => i - 1,
            _ => {
                println!("Invalid choice.");
                return;
            }
        };
        
        let method = self.calculator_factory.get_all_calculators()[index];
        profile.calculation_method = method.to_string();
        println!("Calculation method changed to: {}", method);
    }
    
    /// Displays comprehensive diet and profile statistics for the current date
    /// 
    /// This method provides a detailed statistical overview combining:
    /// 1. Current user profile information (age, gender, height, weight, activity)
    /// 2. Calorie calculation method and target calories for the current date
    /// 3. Food consumption summary with total calories consumed
    /// 4. Diet progress analysis (surplus/deficit, percentage of target achieved)
    /// 
    /// Statistical insights include:
    /// - BMR (Basal Metabolic Rate) calculation
    /// - TDEE (Total Daily Energy Expenditure) based on activity level
    /// - Current calorie consumption vs target comparison
    /// - Diet goal progress indicators
    /// 
    /// Integrates multiple design patterns:
    /// - Repository Pattern: Access to profile and log data
    /// - Strategy Pattern: Flexible calorie calculation methods
    /// - Factory Pattern: Creation of appropriate calculator instances
    fn view_stats(&self) {
        println!("\n------ View Statistics ------");
        
        let profile = match self.profile_repo.get_profile() {
            Some(p) => p,
            None => {
                println!("No profile exists! Please create a profile first.");
                return;
            }
        };
        
        // Get calculator
        let calculator = self.calculator_factory.get_calculator(&profile.calculation_method)
            .unwrap_or_else(|| self.calculator_factory.get_calculator("harris_benedict").unwrap());
        
        // Calculate target calories
        let target_calories = calculator.calculate_target_calories(profile, self.current_date);
        
        println!("Statistics for {}", self.current_date.format("%Y-%m-%d"));
        println!("Target Calories: {:.1}", target_calories);
        
        // Get log for current date
        if let Some(log) = self.log_repo.get_log(self.current_date) {
            let total_calories = log.total_calories(self.food_repo.get_foods());
            
            println!("Total Calories Consumed: {:.1}", total_calories);
            println!("Difference: {:.1}", total_calories - target_calories);
        } else {
            println!("No food logged for today.");
            println!("Total Calories Consumed: 0.0");
            println!("Difference: {:.1}", -target_calories);
        }
        
        // Show weight history if available
        if !profile.daily_profiles.is_empty() {
            println!("\nWeight History:");
            
            // Sort by date
            let mut profiles = profile.daily_profiles.clone();
            profiles.sort_by_key(|p| p.date);
            
            for daily in profiles {
                println!("{}: {:.1} kg", daily.date.format("%Y-%m-%d"), daily.weight);
            }
        }
    }
    
    /// Persists all application data to disk using the Repository Pattern
    /// 
    /// This method coordinates data persistence across all repositories:
    /// 1. Food database persistence (foods.txt) - maintains food definitions
    /// 2. Food logs persistence (logs.txt) - saves daily consumption records
    /// 3. User profile persistence (profile.txt) - stores user information
    /// 
    /// Data persistence features:
    /// - Atomic operations to prevent data corruption
    /// - Error handling with user feedback for failed saves
    /// - Repository Pattern abstraction for consistent data access
    /// - File-based storage for simplicity and portability
    /// 
    /// This method ensures data durability and enables application state
    /// to be maintained across sessions. The Repository Pattern provides
    /// a clean separation between data access logic and business logic,
    /// making the system maintainable and testable.
    fn save_data(&self) {
        println!("Saving data...");
        
        match self.food_repo.save() {
            Ok(_) => println!("Food data saved successfully."),
            Err(e) => println!("Error saving food data: {}", e),
        }
        
        match self.log_repo.save() {
            Ok(_) => println!("Log data saved successfully."),
            Err(e) => println!("Error saving log data: {}", e),
        }
        
        match self.profile_repo.save() {
            Ok(_) => println!("Profile data saved successfully."),
            Err(e) => println!("Error saving profile data: {}", e),
        }
    }
    /// Undoes the last executed command using the Command Pattern
    /// 
    /// This method implements the undo functionality of the Command Pattern:
    /// 1. Checks if there are any commands available to undo
    /// 2. Displays the description of the command being undone
    /// 3. Executes the undo operation through the command manager
    /// 4. Provides feedback on the success or failure of the undo operation
    /// 
    /// Command Pattern benefits:
    /// - Encapsulates operations as objects for easy undo/redo
    /// - Maintains command history for multiple undo levels
    /// - Decouples command execution from command creation
    /// - Enables macro recording and replay capabilities
    /// 
    /// Supported undoable operations include:
    /// - Food additions (basic and composite)
    /// - Food log entries and deletions
    /// - Profile modifications (basic and daily updates)
    /// - Calculation method changes
    fn undo_last_command(&mut self) {
        if !self.command_manager.has_commands_to_undo() {
            println!("No commands to undo.");
            return;
        }
        
        println!("Undoing last command: {}", 
                 self.command_manager.get_command_history().last().unwrap_or(&"Unknown".to_string()));
        
        match self.command_manager.undo_last_command() {
            Ok(_) => println!("Command undone successfully."),
            Err(e) => println!("Error undoing command: {}", e),
        }
    }
    
    /// Deletes a specific food log entry for the current date with user confirmation
    /// 
    /// This method implements safe deletion of food log entries with:
    /// 1. Validation that log entries exist for the current date
    /// 2. User selection of specific entry by number (1-based indexing)
    /// 3. Input validation for entry number bounds checking
    /// 4. Confirmation dialog showing entry details before deletion
    /// 5. Command pattern integration for undo functionality
    /// 
    /// Safety features:
    /// - Bounds checking prevents array index errors
    /// - Confirmation dialog prevents accidental deletions
    /// - User-friendly display shows food name and servings
    /// - Clear feedback on success or failure
    /// 
    /// Uses the Command pattern (RemoveLogEntryCommand) to enable undoing
    /// of deletion operations, maintaining consistency with the application's
    /// command-based architecture for all data modifications.
    fn delete_log_entry(&mut self) {
        println!("\n------ Delete Food Log Entry ------");
        
        // Get log for current date
        let log = match self.log_repo.get_log(self.current_date) {
            Some(log) => log,
            None => {
                println!("No food entries for {}", self.current_date.format("%Y-%m-%d"));
                return;
            }
        };
        
        if log.entries.is_empty() {
            println!("No food entries to delete.");
            return;
        }
        
        print!("Enter the entry number to delete (1-{}): ", log.entries.len());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let entry_number = match input.trim().parse::<usize>() {
            Ok(n) if n >= 1 && n <= log.entries.len() => n,
            _ => {
                println!("Invalid entry number. Please enter a number between 1 and {}.", log.entries.len());
                return;
            }
        };
        
        // Convert to 0-based index
        let index = entry_number - 1;
        
        // Get the entry details for confirmation
        let entry = &log.entries[index];
        let food_name = self.food_repo.get_food(&entry.food_id)
            .map_or("Unknown".to_string(), |f| f.name.clone());
        
        println!("Are you sure you want to delete this entry?");
        println!("Entry {}: {} servings of {} ({})", 
                entry_number, entry.servings, food_name, entry.food_id);
        print!("Type 'yes' to confirm: ");
        io::stdout().flush().unwrap();
        
        let mut confirmation = String::new();
        io::stdin().read_line(&mut confirmation).unwrap();
        
        if confirmation.trim().to_lowercase() != "yes" {
            println!("Delete cancelled.");
            return;
        }
        
        // Create and execute the remove command
        let command = Box::new(RemoveLogEntryCommand::new(
            &mut self.log_repo,
            self.current_date,
            index
        ));
        
        match self.command_manager.execute_command(command) {
            Ok(_) => println!("Food entry deleted successfully!"),
            Err(e) => println!("Error deleting food entry: {}", e),
        }
    }
}

fn main() {
    match App::new() {
        Ok(mut app) => app.run(),
        Err(e) => println!("Error initializing app: {}", e),
    }
}