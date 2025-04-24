# GUI Configuration Improvements

## SimpleThemeInterface Implementation

The SimpleThemeInterface approach separates menu functionality from theme colors, providing a more maintainable and extensible configuration system.

### Core Components

1. **Menu Structure File** (`future_menu.json`)
   - Contains purely functional menu data
   - Defines button behaviors and state transitions
   - Specifies which profiles have which action buttons
   - Tracks current state of each profile

2. **Theme Configuration File** (`ui_colors.json`)
   - Contains all visual styling information
   - Defines colors for different states
   - Specifies button highlighting colors
   - Can be replaced/modified without affecting menu functionality

### Implementation Details

#### Menu Structure Format
```json
{
  "button_types": {
    "start": {
      "applicable_states": ["stopped", "killed"],
      "next_state": "started"
    },
    "freeze": {
      "applicable_states": ["started"],
      "next_state": "frozen"
    },
    "unfreeze": {
      "applicable_states": ["frozen"],
      "next_state": "started" 
    },
    "kill": {
      "applicable_states": ["started", "frozen"],
      "next_state": "killed"
    }
  },
  "profiles": [
    {
      "label": "ProfileName",
      "current_state": "stopped",
      "actions": ["start", "freeze", "unfreeze", "kill"]
    }
  ],
  "groups": [
    {
      "name": "GroupName",
      "profiles": ["ProfileName1", "ProfileName2"]
    }
  ]
}