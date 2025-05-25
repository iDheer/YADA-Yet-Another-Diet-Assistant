# YADA (Yet Another Diet Assistant)

## Overview

YADA is a comprehensive command-line diet and nutrition tracking application written in Rust. Designed with software engineering best practices in mind, it employs multiple design patterns to ensure code maintainability, extensibility, and reusability. The application offers users a robust way to monitor their food intake, track calorie consumption, and analyze dietary patterns against personalized calorie targets.

## 🎯 Key Features

### Comprehensive User Profile Management
- **Personal Information Tracking**: Gender, height, birth date with automatic age calculation
- **Dynamic Daily Profiles**: Weight and activity level tracking with date-specific entries
- **Multiple Calculation Methods**: Harris-Benedict and Mifflin-St Jeor formulas for accurate TDEE calculations
- **Activity Level Support**: Five levels from Sedentary to Extremely Active
- **Profile History**: Track changes over time for improved accuracy

### Advanced Food Database Management
- **Composite Pattern Implementation**: Support for both basic and composite foods
- **Basic Foods**: Simple foods with direct calorie values (e.g., apple, bread)
- **Composite Foods**: Complex foods built from multiple components (e.g., sandwich, recipes)
- **Flexible Search System**: AND/OR keyword-based searching for efficient food discovery
- **Pre-populated Database**: Extensive collection of common foods with proper categorization
- **Extensible Architecture**: Easy addition of new food types and sources

### Daily Food Logging with Complete Management
- **Date-Specific Logging**: Track consumption for any date (past, present, future)
- **Fractional Servings**: Support for precise serving amounts (0.5, 1.5, etc.)
- **Interactive Log Management**: View, add, and **delete** food entries with confirmation
- **Calorie Calculations**: Automatic total calorie computation with target comparison
- **Chronological Tracking**: Timestamped entries for detailed consumption analysis

### Command Pattern with Full Undo Support
- **Complete Undo Functionality**: All data modifications can be undone
- **Command History**: Track all operations with descriptive information
- **Error Recovery**: Safe operation reversal with detailed feedback
- **Configurable History**: Bounded command stack to manage memory usage

### Robust Data Persistence
- **Repository Pattern Implementation**: Clean separation of data access from business logic
- **File-Based Storage**: Simple, portable text file format for all data
- **Automatic Data Loading**: Seamless restoration of application state on startup
- **Manual Save Options**: User-controlled data persistence with error handling
- **Data Integrity**: Robust error handling for file operations

## 🏗️ Software Architecture

### Design Patterns Implementation

#### Command Pattern
- **Complete Undo/Redo System**: All data modifications encapsulated as command objects
- **Command Interface**: Consistent `execute()` and `undo()` methods across all operations
- **Command Manager**: Centralized execution with automatic undo stack management
- **Command Types**: Categorized operations (AddFood, RemoveFood, AddLog, DeleteLog, UpdateProfile)
- **Bounded History**: Configurable command stack size to prevent unlimited memory growth

#### Composite Pattern
- **Food Hierarchy**: Unified interface for basic and composite food types
- **Recursive Structure**: Composite foods can contain other composite foods
- **Automatic Calculations**: Calorie values computed recursively from components
- **Uniform Treatment**: Same interface for simple foods and complex recipes

#### Repository Pattern
- **Data Access Abstraction**: Clean separation between business logic and storage
- **Consistent CRUD Operations**: Uniform interface across all data types
- **Three Repositories**: 
  - `FoodRepository`: Food database management
  - `LogRepository`: Daily consumption tracking
  - `ProfileRepository`: User profile persistence

#### Strategy Pattern
- **Calorie Calculation Strategies**: Interchangeable algorithms for BMR/TDEE calculations
- **Runtime Selection**: Users can switch calculation methods at runtime
- **Extensible Design**: Easy addition of new calculation formulas
- **Consistent Interface**: All strategies implement the same calculation contract

#### Factory Pattern
- **Calculator Factory**: Creates appropriate calorie calculation strategy instances
- **Food Source Factory**: Manages creation of different food data sources
- **Centralized Creation**: Consistent object instantiation across the application
### Core Components

#### Models (`src/models/`)
- **`food.rs`**: Composite Pattern implementation for basic and composite foods
- **`log.rs`**: Daily food consumption tracking with timestamped entries
- **`profile.rs`**: User profile management with basic and daily profile components
- **`command.rs`**: Command Pattern trait definition with error handling
- **`command_manager.rs`**: Command execution and undo management system

#### Repositories (`src/repositories/`)
- **`food_repository.rs`**: Food database management with search capabilities
- **`log_repository.rs`**: Daily log persistence with date-based organization
- **`profile_repository.rs`**: User profile storage with validation

#### Commands (`src/commands/`)
- **`food_commands.rs`**: Food database modification commands (Add, Update, Remove)
- **`log_commands.rs`**: Food logging commands (Add, Remove log entries)
- **`profile_commands.rs`**: Profile management commands (Basic, Daily updates)

#### Strategies (`src/strategies/`)
- **`calorie_calculator.rs`**: BMR/TDEE calculation strategies (Harris-Benedict, Mifflin-St Jeor)

#### Factories (`src/factories/`)
- **`food_source_factory.rs`**: Food source creation and management system

## 📁 Project Structure
```
yada/
├── Cargo.toml                   # Project configuration and dependencies
├── foods.txt                    # Pre-populated food database
└── src/
    ├── main.rs                  # Application entry point with comprehensive UI
    ├── models/                  # Core data structures
    │   ├── mod.rs              # Module organization with design pattern docs
    │   ├── food.rs             # Composite Pattern food implementation
    │   ├── log.rs              # Daily consumption tracking models
    │   ├── profile.rs          # User profile with daily tracking
    │   ├── command.rs          # Command Pattern trait definition
    │   └── command_manager.rs  # Command execution and undo system
    ├── repositories/           # Data persistence layer (Repository Pattern)
    │   ├── mod.rs              # Repository module organization
    │   ├── food_repository.rs  # Food database management
    │   ├── log_repository.rs   # Consumption log persistence
    │   └── profile_repository.rs # User profile storage
    ├── commands/               # Command Pattern implementations
    │   ├── mod.rs              # Command module organization
    │   ├── food_commands.rs    # Food management commands
    │   ├── log_commands.rs     # Log entry commands
    │   └── profile_commands.rs # Profile modification commands
    ├── strategies/             # Strategy Pattern implementations
    │   ├── mod.rs              # Strategy module organization
    │   └── calorie_calculator.rs # Calculation method strategies
    └── factories/              # Factory Pattern implementations
        ├── mod.rs              # Factory module organization
        └── food_source_factory.rs # Food source creation
```

## 🚀 Installation and Setup

### Prerequisites
- **Rust Toolchain**: Version 1.70.0 or later
- **Cargo Package Manager**: Included with Rust installation

### Installation Steps
1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/yada.git
   cd yada
   ```

2. **Build the application**:
   ```bash
   cargo build --release
   ```

3. **Run YADA**:
   ```bash
   cargo run --release
   ```

### Alternative Installation
```bash
cargo install yada  # Install from crates.io (when published)
```

## 📖 Usage Guide

### First-Time Setup
When you first run YADA, you'll be guided through profile creation:

```
Welcome to YADA (Yet Another Diet Assistant)!
Creating your user profile...

Select your gender:
1. Male
2. Female  
3. Other
Enter your choice (1-3): 2

Enter your height in cm: 165
Enter your birth date (YYYY-MM-DD): 1990-05-15
Enter your current weight in kg: 62.5

Select your activity level:
1. Sedentary (little or no exercise)
2. Lightly active (light exercise/sports 1-3 days/week)
3. Moderately active (moderate exercise/sports 3-5 days/week)
4. Very active (hard exercise/sports 6-7 days a week)
5. Extremely active (very hard exercise & physical job or training twice a day)
Enter your choice (1-5): 3

Profile created successfully!
Your daily calorie target: 2,187 calories
```

### Main Menu Navigation

YADA provides an intuitive menu-driven interface:

```
====== YADA - Yet Another Diet Assistant ======
Current Date: Wednesday, May 25, 2025

1. Change Date               # Navigate to different dates
2. Search Foods             # Find foods in the database
3. Create User Profile      # Set up or recreate profile
4. Manage Foods            # Add basic or composite foods
5. View Food Database      # Browse all available foods
6. Log Food Consumption    # Record what you've eaten
7. View Food Log          # See daily consumption with delete option
8. Manage Profile         # Update profile information
9. View Statistics        # See nutritional analysis
10. Save Data             # Manually save all changes
11. Undo Last Command     # Reverse last operation
12. Exit                  # Close application

Enter your choice (1-12):
```

### Key Operations

#### Food Database Management
```
# Adding a basic food
Manage Foods > Add Basic Food
Enter food ID: banana
Enter food name: Banana
Enter keywords: fruit,sweet,yellow
Enter calories per serving: 105

# Creating a composite food (recipe)
Manage Foods > Create Composite Food
Enter food ID: fruit_salad
Enter food name: Fruit Salad
Enter keywords: fruit,healthy,mixed
Adding components... (add banana: 1.0, apple: 2.0, etc.)
```

#### Daily Food Logging
```
# Logging food consumption
Log Food Consumption > (search for "banana")
Found: Banana (105 cal/serving)
Enter servings consumed: 1.5
Successfully logged 1.5 servings of Banana (157.5 calories)

# Viewing and managing food log
View Food Log
Food log for Wednesday, May 25, 2025:
#  Food ID    Name      Servings  Calories
1  banana     Banana    1.5       157.5
2  apple      Apple     1.0       52.0
Total calories: 209.5
Target calories: 2,187.0
Difference: -1,977.5

Options:
1. Delete a food entry    # NEW: Remove specific entries
2. Back to main menu
```

#### Profile Management
```
# Updating profile information
Manage Profile > Update Basic Profile
Current height: 165.0 cm
Enter new height (or leave blank): 167
Profile updated successfully!

# Updating daily information
Manage Profile > Update Today's Data
Enter current weight: 63.2
Select activity level: 4 (Very Active)
Daily profile updated!
```

### Advanced Features

#### Undo Functionality
```
# All operations can be undone
Undo Last Command
Undoing last command: Add Log Entry (Banana, 1.5 servings)
Command undone successfully.
```

#### Food Search System
```
# Flexible search with AND/OR logic
Search Foods
Enter keywords (comma-separated): fruit,sweet
Search type:
1. Match ALL keywords (AND)
2. Match ANY keyword (OR)
Enter choice: 1

Found 3 foods matching ALL keywords:
1. Apple (52 cal/serving)
2. Banana (105 cal/serving)  
3. Orange (62 cal/serving)
```

#### Statistics and Analysis
```
View Statistics
User Profile Statistics:
Age: 35 years
Gender: Female
Height: 167.0 cm
Current Weight: 63.2 kg
Activity Level: Very Active
BMR: 1,482 calories
TDEE: 2,298 calories

Daily Consumption (May 25, 2025):
Total Calories: 209.5
Target Calories: 2,298.0
Remaining: 2,088.5 calories
Progress: 9.1% of daily target
```

## Troubleshooting

### Common Issues

#### Application won't start
- Ensure Rust is properly installed: `rustc --version`
- Check if the data directory has proper permissions

#### Data not saving
- Verify write permissions to the application directory
- Try using the manual `save` command before exiting

#### Incorrect calorie calculations
- Ensure your profile information is accurate and complete
- Check if you've selected the appropriate calculation method

### Data Recovery
If your data files become corrupted, you can find backups in the `.yada/backups` directory. The application automatically creates daily backups of your data.

## Contributing

We welcome contributions to YADA! Here's how you can help:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

Please ensure your code adheres to our coding standards and includes appropriate tests.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/yada.git
cd yada

# Install development dependencies
cargo install cargo-watch cargo-tarpaulin

# Run tests in watch mode
cargo watch -x test

# Check code coverage
cargo tarpaulin
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- The Harris-Benedict and Mifflin-St Jeor equations are based on published scientific research
- Inspired by various nutrition tracking applications but built from the ground up with software engineering best practices
