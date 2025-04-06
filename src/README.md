# YADA (Yet Another Diet Assistant)

## Overview

YADA is a comprehensive command-line diet and nutrition tracking application written in Rust. Designed with software engineering best practices in mind, it offers users a robust way to monitor their food intake, track calorie consumption, and analyze dietary patterns against personalized calorie targets. The application employs various design patterns to ensure code maintainability, extensibility, and reusability.

## Features

### User Profile Management
- Create and update detailed user profiles with personal data:
  - Gender (Male, Female, Other)
  - Height (in centimeters)
  - Birth date (for age calculation)
  - Weight tracking over time
  - Activity level (from Sedentary to Extremely Active)
- Calculate age automatically based on birth date and current date
- Support for multiple calculation methods for determining daily calorie targets
- BMR (Basal Metabolic Rate) calculation using different formulas
- TDEE (Total Daily Energy Expenditure) estimation based on activity level

### Food Database Management
- Extensive food database with pre-populated basic foods
- Add custom basic foods with:
  - Unique identifiers
  - Descriptive names
  - Searchable keywords
  - Accurate calorie content per serving
- Create composite foods (recipes) from existing foods with:
  - Component foods from the database
  - Customizable serving amounts for each component
  - Automatic calorie calculation based on components
- Sophisticated search functionality:
  - Keyword-based searching
  - Support for AND search (match all keywords)
  - Support for OR search (match any keywords)
- Comprehensive food database viewing and browsing

### Daily Food Logging
- Log food consumption with:
  - Date-specific entries
  - Customizable serving sizes
  - Automatic calorie calculation
- View daily food consumption with:
  - Itemized food entries
  - Serving amounts
  - Calorie breakdown per item
  - Total daily calorie sum
- Track historical food intake across different dates
- Compare actual calorie intake against personalized calorie targets
- Calculate calorie surplus/deficit

### Nutritional Target Calculation
- Multiple scientific formulas for calculating calorie needs:
  - Harris-Benedict Equation (original and revised)
  - Mifflin-St Jeor Equation
  - Extensible system for adding other calculation methods
- Adjustment of calorie targets based on:
  - Gender
  - Age
  - Weight
  - Height
  - Activity level

### Date Management
- Change the working date to:
  - View historical food logs
  - Add food entries to past or future dates
  - Track weight changes over time
- Support for viewing and analyzing historical data
- Easy reset to current date

### Command History and Undo Functionality
- Complete command history tracking
- Undo capability for all operations:
  - Adding foods
  - Logging food consumption
  - Updating profile information
- Protection against accidental data loss
- Informative command descriptions

### Data Persistence
- Automatic data storage in text files:
  - `foods.txt`: Food database
  - `logs.txt`: Food consumption logs
  - `profile.txt`: User profile data
- Automatic loading of saved data on startup
- Manual save option to ensure data is preserved
- Error handling for file operations

## Technical Implementation

### Design Patterns

#### Command Pattern
- All user actions encapsulated as command objects
- Command interface with `execute()` and `undo()` methods
- Command history maintained for tracking and undoing actions
- Command types categorized for better organization
- Each command provides descriptive information

#### Factory Pattern
- `FoodSourceFactory` for creating and managing different food sources
- `CalorieCalculatorFactory` for instantiating different calorie calculation strategies
- Centralized creation logic for related objects
- Runtime selection of appropriate implementations

#### Strategy Pattern
- Interchangeable algorithms for calculating daily calorie needs
- Selection of calculation method at runtime
- Consistent interface across different calculation methods
- Easy addition of new calculation strategies

#### Repository Pattern
- Dedicated data access layer with repositories for:
  - Food database
  - Food logs
  - User profiles
- Separation of storage details from business logic
- Consistent CRUD operations across different data types

#### Composite Pattern
- Representation of foods as either basic or composite structures
- Uniform treatment of individual foods and food collections
- Recursive calculation of nutritional values for composite foods
- Ability to build complex food items from simpler components

### Core Components

#### Models
- `Food`: Represents basic and composite food items
- `LogEntry` and `DailyLog`: Track food consumption
- `UserProfile` and `DailyProfile`: Store user information
- `Command`: Interface for implementing the Command pattern
- `CommandManager`: Manages command execution and history

#### Repositories
- `FoodRepository`: Manages the food database
- `LogRepository`: Handles food consumption logs
- `ProfileRepository`: Stores user profile information

#### Commands
- `AddFoodCommand`: Adds a new food to the database
- `AddLogEntryCommand`: Records food consumption
- `UpdateUserProfileCommand`: Updates user profile information
- `UpdateDailyProfileCommand`: Updates daily weight and activity

#### Strategies
- `HarrisBenedictCalculator`: Implements the Harris-Benedict equation
- `MifflinStJeorCalculator`: Implements the Mifflin-St Jeor equation
- `CalorieCalculator`: Common interface for all calculation methods

#### Factories
- `CalorieCalculatorFactory`: Creates appropriate calculator instances
- `FoodSourceFactory`: Creates and manages food data sources

## File Structure
```
src/
├── main.rs                     # Application entry point
├── models/                     # Core data structures
│   ├── food.rs                 # Food model definitions
│   ├── log.rs                  # Food log structures
│   ├── profile.rs              # User profile models
│   └── mod.rs                  # Module exports
├── commands/                   # Command pattern implementation
│   ├── food_commands.rs        # Food database commands
│   ├── log_commands.rs         # Food logging commands
│   ├── profile_commands.rs     # Profile management commands
│   ├── date_commands.rs        # Date handling commands
│   └── mod.rs                  # Command module exports
├── repositories/               # Data persistence layer
│   ├── food_repository.rs      # Food data storage
│   ├── log_repository.rs       # Log data storage
│   ├── profile_repository.rs   # Profile data storage
│   └── mod.rs                  # Repository module exports
├── strategies/                 # Strategy pattern implementations
│   ├── calorie_calculators.rs  # BMR/TDEE calculation strategies
│   ├── search_strategies.rs    # Food search implementations
│   └── mod.rs                  # Strategy module exports
├── factories/                  # Factory pattern implementations
│   ├── calculator_factory.rs   # Calorie calculator creation
│   ├── food_factory.rs         # Food object creation
│   └── mod.rs                  # Factory module exports
└── utils/                      # Helper functions and utilities
    ├── date_utils.rs           # Date manipulation helpers
    ├── parsing.rs              # String parsing utilities
    ├── formatting.rs           # Output formatting helpers
    └── mod.rs                  # Utility module exports
```

## Installation

### Prerequisites
- Rust toolchain (1.51.0 or later)
- Cargo package manager

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/yada.git
   cd yada
   ```

2. Build the application:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

### Installation from Crates.io
```bash
cargo install yada
```

## Usage

### First-time Setup
When you first run YADA, you'll be prompted to create a user profile:

```
Welcome to YADA (Yet Another Diet Assistant)!
Let's set up your profile.

Enter your gender (Male/Female/Other): Female
Enter your height in cm: 165
Enter your birthdate (YYYY-MM-DD): 1990-05-15
Enter your current weight in kg: 62.5
Select your activity level:
1. Sedentary (little or no exercise)
2. Lightly active (light exercise/sports 1-3 days/week)
3. Moderately active (moderate exercise/sports 3-5 days/week)
4. Very active (hard exercise/sports 6-7 days a week)
5. Extremely active (very hard exercise, physical job or training twice a day)
Enter your choice (1-5): 3
```

### Basic Commands

Here are some common commands you can use in YADA:

#### User Profile Management
```
profile show                # Display current user profile
profile update height 168   # Update height to 168 cm
profile update weight 64.2  # Update current weight to 64.2 kg
profile update activity 4   # Change activity level to Very Active
```

#### Food Database Management
```
food add "Apple" "fruit, sweet" 52          # Add a basic food with keywords and calories per serving
food add-composite "Fruit Salad" "Apple:2 Banana:1 Orange:1"  # Create a composite food
food search apple                           # Search for foods containing "apple"
food search "fruit AND sweet"               # Search using AND operator
food list                                   # List all foods in database
```

#### Food Logging
```
log add "Apple" 2           # Log 2 servings of Apple for the current date
log show                    # Display food log for the current date
log show 2023-09-15         # Display food log for a specific date
log summary week            # Show a weekly summary of your food logs
```

#### Date Management
```
date set 2023-09-15         # Change working date
date reset                  # Reset to current date
date show                   # Show current working date
```

#### Command History
```
history                     # Show command history
undo                        # Undo the last command
```

#### System Commands
```
save                        # Manually save all data
help                        # Display help information
exit                        # Exit the application
```

## Advanced Features

### Custom Calorie Calculation Methods
YADA allows you to choose between different scientific formulas for calculating your daily calorie needs:

```
profile set-calculator harris-benedict  # Use the Harris-Benedict equation
profile set-calculator mifflin-stjor    # Use the Mifflin-St Jeor equation
```

### Nutritional Analysis
```
stats daily                 # Show daily nutritional statistics
stats weekly                # Show weekly trends
stats monthly               # Show monthly trends
stats compare target        # Compare intake against targets
```

### Data Export
```
export logs csv 2023-09-01 2023-09-30  # Export logs for date range to CSV
export profile txt                      # Export profile data to text file
```

## Example Session

Here's an example of a typical YADA session:

```
> profile show
User Profile:
  Gender: Female
  Height: 165 cm
  Age: 33 (DOB: 1990-05-15)
  Current Weight: 62.5 kg
  Activity Level: Moderately Active
  Calculated Daily Calorie Target: 2187 calories

> food search fruit
Found 4 foods matching your search:
  1. Apple (52 cal/serving) - Keywords: fruit, sweet
  2. Banana (105 cal/serving) - Keywords: fruit, sweet
  3. Orange (62 cal/serving) - Keywords: fruit, citrus
  4. Fruit Salad (271 cal/serving) - Composite food

> log add "Apple" 1
Added 1 serving of Apple (52 calories) to your log for 2023-10-25.

> log add "Banana" 1.5
Added 1.5 servings of Banana (157.5 calories) to your log for 2023-10-25.

> log show
Food Log for Wednesday, October 25, 2023:
  - Apple: 1 serving (52 calories)
  - Banana: 1.5 servings (157.5 calories)
  Total: 209.5 calories (1977.5 below daily target)
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
- Special thanks to all contributors who have helped improve YADA
- Inspired by various nutrition tracking applications but built from the ground up with software engineering best practices
