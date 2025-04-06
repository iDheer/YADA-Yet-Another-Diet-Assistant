// src/main.rs
mod models;
mod repositories;
mod commands;
mod strategies;
mod factories;

use std::io::{self, Write};
use std::collections::HashSet;
use chrono::{Local, NaiveDate};

use models::command_manager::CommandManager;
use models::profile::{Gender, ActivityLevel, UserProfile, DailyProfile};
use models::food::Food;

use repositories::food_repository::FoodRepository;
use repositories::log_repository::LogRepository;
use repositories::profile_repository::ProfileRepository;

use commands::food_commands::AddFoodCommand;
use commands::log_commands::AddLogEntryCommand;
use commands::profile_commands::{UpdateUserProfileCommand, UpdateDailyProfileCommand};

use strategies::calorie_calculator::CalorieCalculatorFactory;

use factories::food_source_factory::FoodSourceFactory;

enum MenuOption {
    ManageFood,
    ViewFood,
    LogFood,
    ViewLog,
    ManageProfile,
    ViewStats,
    ChangeDate,
    SaveData,
    Undo,
    Exit,
}

struct App {
    food_repo: FoodRepository,
    log_repo: LogRepository,
    profile_repo: ProfileRepository,
    command_manager: CommandManager,
    calculator_factory: CalorieCalculatorFactory,
    food_source_factory: FoodSourceFactory,
    current_date: NaiveDate,
}

impl App {
    fn new() -> Result<Self, io::Error> {
        let food_repo = FoodRepository::new("foods.txt")?;
        let log_repo = LogRepository::new("logs.txt")?;
        let profile_repo = ProfileRepository::new("profile.txt")?;
        
        let command_manager = CommandManager::new(100); // Store up to 100 commands
        let calculator_factory = CalorieCalculatorFactory::new();
        let food_source_factory = FoodSourceFactory::new();
        
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
        
        // Seed the database with initial foods if it's empty
        if app.food_repo.get_all_foods().is_empty() {
            app.seed_initial_foods();
        }
        
        Ok(app)
    }
    
    fn seed_initial_foods(&mut self) {
        println!("Initializing food database with basic foods...");
        
        // Helper function for adding basic foods - Add type annotations here
        let mut add_basic_food = |id: &str, name: &str, keywords: &str, calories: f64| {
            let kw_set: HashSet<String> = keywords.split(',').map(|s| s.trim().to_lowercase().to_string()).collect();
            let food = Food::new_basic(id.to_string(), name.to_string(), kw_set, calories);
            self.food_repo.add_food(food).ok();
        };
        
        // Dairy
        add_basic_food("milk_whole", "Whole Milk (1 cup)", "milk,dairy,drink", 150.0);
        add_basic_food("milk_skim", "Skim Milk (1 cup)", "milk,dairy,drink,skim", 90.0);
        add_basic_food("cheese_cheddar", "Cheddar Cheese (1 oz)", "cheese,dairy,cheddar", 110.0);
        add_basic_food("yogurt_plain", "Plain Yogurt (1 cup)", "yogurt,dairy", 120.0);
        
        // Meat & Protein
        add_basic_food("chicken_breast", "Chicken Breast (4 oz)", "chicken,meat,protein", 170.0);
        add_basic_food("beef_ground", "Ground Beef 85% (4 oz)", "beef,meat,protein", 240.0);
        add_basic_food("eggs", "Eggs (1 large)", "eggs,protein", 70.0);
        add_basic_food("tuna", "Tuna (1 can)", "tuna,fish,protein", 180.0);
        
        // Fruits
        add_basic_food("apple", "Apple (medium)", "apple,fruit", 95.0);
        add_basic_food("banana", "Banana (medium)", "banana,fruit", 105.0);
        add_basic_food("orange", "Orange (medium)", "orange,fruit,citrus", 65.0);
        add_basic_food("strawberries", "Strawberries (1 cup)", "strawberry,fruit,berries", 50.0);
        
        // Vegetables
        add_basic_food("broccoli", "Broccoli (1 cup)", "broccoli,vegetable,veggie", 55.0);
        add_basic_food("carrot", "Carrot (medium)", "carrot,vegetable,veggie", 25.0);
        add_basic_food("spinach", "Spinach (1 cup)", "spinach,vegetable,veggie,leafy", 7.0);
        add_basic_food("potato", "Potato (medium)", "potato,vegetable,starchy", 110.0);
        
        // Grains
        add_basic_food("bread_wheat", "Wheat Bread (1 slice)", "bread,grain,wheat", 80.0);
        add_basic_food("rice_white", "White Rice (1 cup cooked)", "rice,grain,white", 200.0);
        add_basic_food("pasta", "Pasta (1 cup cooked)", "pasta,grain", 220.0);
        add_basic_food("oatmeal", "Oatmeal (1 cup cooked)", "oatmeal,grain,breakfast", 160.0);
        
        // Other
        add_basic_food("peanut_butter", "Peanut Butter (2 tbsp)", "peanut,butter,spread", 190.0);
        add_basic_food("jelly", "Grape Jelly (1 tbsp)", "jelly,grape,spread", 50.0);
        add_basic_food("olive_oil", "Olive Oil (1 tbsp)", "oil,fat", 120.0);
        add_basic_food("soda", "Soda (12 oz can)", "soda,drink,sugar", 150.0);
        
        // Create some composite foods
        let mut pb_sandwich = Food::new_composite(
            "pb_sandwich".to_string(),
            "Peanut Butter Sandwich".to_string(),
            ["sandwich", "peanut butter", "lunch"].iter().map(|s| s.to_string()).collect(),
            vec![("bread_wheat".to_string(), 2.0), ("peanut_butter".to_string(), 1.0)]
        );
        
        // Calculate calories for composite food
        let mut total_calories = 0.0;
        for (comp_id, servings) in &pb_sandwich.components {
            if let Some(component) = self.food_repo.get_food(comp_id) {
                total_calories += component.calories_per_serving * servings;
            }
        }
        pb_sandwich.calories_per_serving = total_calories;
        self.food_repo.add_food(pb_sandwich).ok();
        
        // PB&J sandwich
        let mut pbj_sandwich = Food::new_composite(
            "pbj_sandwich".to_string(),
            "PB&J Sandwich".to_string(),
            ["sandwich", "peanut butter", "jelly", "lunch"].iter().map(|s| s.to_string()).collect(),
            vec![("pb_sandwich".to_string(), 1.0), ("jelly".to_string(), 1.0)]
        );
        
        // Calculate calories for composite food
        let mut total_calories = 0.0;
        for (comp_id, servings) in &pbj_sandwich.components {
            if let Some(component) = self.food_repo.get_food(comp_id) {
                total_calories += component.calories_per_serving * servings;
            }
        }
        pbj_sandwich.calories_per_serving = total_calories;
        self.food_repo.add_food(pbj_sandwich).ok();
        
        println!("Food database initialized with {} basic foods and 2 composite foods.", 24);
        
        // Save the seeded database
        if let Err(e) = self.food_repo.save() {
            println!("Warning: Failed to save seeded food database: {}", e);
        }
    }
    
    fn run(&mut self) {
        println!("Welcome to YADA (Yet Another Diet Assistant)!");
        
        // Check if we have a user profile
        if self.profile_repo.get_profile().is_none() {
            println!("No user profile found. Let's create one!");
            self.create_initial_profile();
        }
        
        loop {
            match self.show_main_menu() {
                MenuOption::ManageFood => self.manage_foods(),
                MenuOption::ViewFood => self.view_foods(),
                MenuOption::LogFood => self.log_food(),
                MenuOption::ViewLog => self.view_log(),
                MenuOption::ManageProfile => self.manage_profile(),
                MenuOption::ViewStats => self.view_stats(),
                MenuOption::ChangeDate => self.change_date(), // Added new menu option
                MenuOption::SaveData => self.save_data(),
                MenuOption::Undo => self.undo_last_command(),
                MenuOption::Exit => {
                    self.save_data();
                    println!("Goodbye!");
                    break;
                }
            }
        }
    }
    
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
    
    // Added new method for changing date
    fn change_date(&mut self) {
        println!("\n------ Change Current Date ------");
        println!("Current date: {}", self.current_date.format("%Y-%m-%d"));
        
        loop {
            print!("Enter new date (YYYY-MM-DD) or 'today' for current date: ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            
            if input.to_lowercase() == "today" {
                self.current_date = Local::now().date_naive();
                println!("Date set to today: {}", self.current_date.format("%Y-%m-%d"));
                break;
            } else {
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
    
    // Added method to search foods
    fn search_foods(&self) -> Vec<&Food> {
        println!("\n------ Search Foods ------");
        
        print!("Enter search keywords (comma-separated): ");
        io::stdout().flush().unwrap();
        
        let mut keywords_str = String::new();
        io::stdin().read_line(&mut keywords_str).unwrap();
        
        let keywords: HashSet<String> = keywords_str
            .trim()
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();
        
        if keywords.is_empty() {
            println!("No valid keywords entered. Returning all foods.");
            return self.food_repo.get_all_foods();
        }
        
        println!("Match all keywords or any keyword?");
        println!("1. Match ANY keyword (OR search)");
        println!("2. Match ALL keywords (AND search)");
        
        print!("Enter your choice (1-2): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let match_all = match input.trim().parse::<u32>() {
            Ok(1) => false,
            Ok(2) => true,
            _ => {
                println!("Invalid choice. Using ANY keyword matching.");
                false
            }
        };
        
        // Find all foods that match the criteria
        let mut results = Vec::new();
        
        for food in self.food_repo.get_all_foods() {
            let matches = if match_all {
                // AND search - all keywords must match
                keywords.iter().all(|k| food.keywords.contains(k))
            } else {
                // OR search - any keyword must match
                keywords.iter().any(|k| food.keywords.contains(k))
            };
            
            if matches {
                results.push(food);
            }
        }
        
        println!("Found {} foods matching your search criteria.", results.len());
        
        results
    }
    
    fn create_initial_profile(&mut self) {
        println!("\n------ Create User Profile ------");
        
        // Gender
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
        
        // Height
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
        
        // Birth date
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
        
        // Create the profile
        let mut profile = UserProfile::new(gender, height, birth_date);
        
        // Add current day's data
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
        
        let daily_profile = DailyProfile {
            date: self.current_date,
            weight,
            activity_level,
        };
        
        profile.add_or_update_daily_profile(daily_profile);
        
        // Set the profile
        self.profile_repo.set_profile(profile);
        println!("Profile created successfully!");
    }
    
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
                Ok(1) => self.add_basic_food(),
                Ok(2) => self.create_composite_food(),
                Ok(3) => break,
                _ => println!("Invalid choice. Please enter a number between 1 and 3."),
            }
        }
    }
    
    fn add_basic_food(&mut self) {
        println!("\n------ Add Basic Food ------");
        
        // Get food details
        print!("Enter food ID (no spaces): ");
        io::stdout().flush().unwrap();
        let mut id = String::new();
        io::stdin().read_line(&mut id).unwrap();
        id = id.trim().to_string();
        
        // Check if ID already exists
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
        
        // Create food and add to repository using the command pattern
        let food = Food::new_basic(id, name, keywords, calories);
        let command = Box::new(AddFoodCommand::new(&mut self.food_repo, food));
        
        match self.command_manager.execute_command(command) {
            Ok(_) => println!("Food added successfully!"),
            Err(e) => println!("Error adding food: {}", e),
        }
    }
    
    fn create_composite_food(&mut self) {
        println!("\n------ Create Composite Food ------");
        
        // Get food details
        print!("Enter food ID (no spaces): ");
        io::stdout().flush().unwrap();
        let mut id = String::new();
        io::stdin().read_line(&mut id).unwrap();
        id = id.trim().to_string();
        
        // Check if ID already exists
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
        
        // Get components
        let mut components: Vec<(String, f64)> = Vec::new();
        
        println!("Add components (enter empty food ID to finish):");
        loop {
            print!("Enter component food ID: ");
            io::stdout().flush().unwrap();
            let mut comp_id = String::new();
            io::stdin().read_line(&mut comp_id).unwrap();
            comp_id = comp_id.trim().to_string();
            
            if comp_id.is_empty() {
                break;
            }
            
            // Check if component exists
            if self.food_repo.get_food(&comp_id).is_none() {
                println!("Food with ID '{}' doesn't exist.", comp_id);
                continue;
            }
            
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
            
            components.push((comp_id, servings));
        }
        
        if components.is_empty() {
            println!("No components added. Cannot create composite food.");
            return;
        }
        
        // Create food
        let food = Food::new_composite(id, name, keywords, components);
        let command = Box::new(AddFoodCommand::new(&mut self.food_repo, food));
        
        match self.command_manager.execute_command(command) {
            Ok(_) => println!("Composite food added successfully!"),
            Err(e) => println!("Error adding composite food: {}", e),
        }
    }
    
    fn view_foods(&self) {
        println!("\n------ View Foods ------");
        
        let foods = self.food_repo.get_all_foods();
        
        if foods.is_empty() {
            println!("No foods in database.");
            return;
        }
        
        println!("{:<10} {:<20} {:<30} {:<10}", "ID", "Name", "Keywords", "Calories");
        println!("{:-<75}", "");
        
        for food in foods {
            let keywords_str = food.keywords.iter().cloned().collect::<Vec<_>>().join(", ");
            println!("{:<10} {:<20} {:<30} {:<10.1}", 
                    food.id, food.name, keywords_str, food.calories_per_serving);
        }
    }
    
    // Updated to include search functionality
    fn log_food(&mut self) {
        println!("\n------ Log Food Consumption ------");
        
        // Check if there are any foods
        let foods = self.food_repo.get_all_foods();
        if foods.is_empty() {
            println!("No foods in database. Please add foods first.");
            return;
        }
        
        println!("1. Show all foods");
        println!("2. Search foods by keyword");
        
        print!("Enter your choice (1-2): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let selected_foods = match input.trim().parse::<u32>() {
            Ok(1) => self.food_repo.get_all_foods(),
            Ok(2) => self.search_foods(),
            _ => {
                println!("Invalid choice. Showing all foods.");
                self.food_repo.get_all_foods()
            }
        };
        
        if selected_foods.is_empty() {
            println!("No foods found.");
            return;
        }
        
        // Show selected foods
        println!("\nAvailable foods:");
        println!("{:<10} {:<20} {:<10}", "ID", "Name", "Calories");
        println!("{:-<45}", "");
        
        for food in &selected_foods {
            println!("{:<10} {:<20} {:<10.1}", 
                    food.id, food.name, food.calories_per_serving);
        }
        
        // Get food ID
        print!("\nEnter food ID: ");
        io::stdout().flush().unwrap();
        let mut food_id = String::new();
        io::stdin().read_line(&mut food_id).unwrap();
        food_id = food_id.trim().to_string();
        
        // Check if food exists
        if self.food_repo.get_food(&food_id).is_none() {
            println!("Food with ID '{}' doesn't exist.", food_id);
            return;
        }
        
        // Get servings
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
        
        // Add log entry using the command pattern
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
    
    fn view_log(&self) {
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
        } else {
            println!("No food entries for {}", self.current_date.format("%Y-%m-%d"));
        }
    }
    
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
}

fn main() {
    match App::new() {
        Ok(mut app) => app.run(),
        Err(e) => println!("Error initializing app: {}", e),
    }
}