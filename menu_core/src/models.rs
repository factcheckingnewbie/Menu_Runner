/// menu_core/src/models.rs
/// Represents a single menu command entry
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Clone)]
pub struct MenuCommand {
    pub name: String,
    pub command: String,
}

/// Represents a grouped menu entry with multiple actions
pub struct GroupedMenuEntry {
    pub program: String,
    pub path_name: String,
    pub actions: Vec<String>,
    pub commands: Vec<String>,
}

/// Represents detailed command information with category and description
/// #[derive(Clone)]
#[derive(Clone, Serialize, Deserialize)]
pub struct CommandInfo {
    pub name: String,
    pub command: String,
    pub description: String,
    pub category: String,
}

/// Represents a menu entry for the Slint UI
#[derive(Clone)]
pub struct SlintMenuEntry {
    pub label: String,
    pub actions: Vec<String>,
    pub command_template: String,
}

/// State display properties
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StateDisplay {
    pub bg: String,
    pub fg: String,
}

/// State definition with display properties and transitions
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct State {
    pub display: StateDisplay,
    pub transitions: HashMap<String, String>,
}

/// State machine definition
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StateMachine {
    pub initial_state: String,
    pub states: HashMap<String, State>,
}

/// Menu item config with state machine
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MenuItemConfig {
    pub command_template: String,
    pub label: String,
    pub state_machine: StateMachine,
}

/// Complete menu configuration
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MenuConfig {
    pub menu_items: Vec<MenuItemConfig>,
}

/// Button state manager to track button visual states
#[derive(Clone)]
pub struct ButtonManager {
    // Maps "profile:button" -> state (true = down/active)
    pub button_states: HashMap<String, bool>,
    
    // Maps "profile" -> current_state
    pub profile_states: HashMap<String, String>,
    
    // Cache of button colors
    pub button_colors: HashMap<String, String>,
    
    // Maps button key to list of buttons it affects
    pub button_affects: HashMap<String, Vec<String>>,
    
    // Reference to menu configuration
    pub menu_config: Option<MenuConfig>,
}

impl ButtonManager {
    pub fn new() -> Self {
        ButtonManager {
            button_states: HashMap::new(),
            profile_states: HashMap::new(),
            button_colors: HashMap::new(),
            button_affects: HashMap::new(),
            menu_config: None,
        }
    }
    
    // Initialize from menu config
    pub fn from_menu_config(config: MenuConfig) -> Self {
        let mut manager = Self::new();
        manager.menu_config = Some(config.clone());
        
        // Initialize states and colors for all menu items
        for item in &config.menu_items {
            // Set initial state for this profile
            manager.profile_states.insert(item.label.clone(), item.state_machine.initial_state.clone());
            
            // Set initial colors based on initial state
            if let Some(state) = item.state_machine.states.get(&item.state_machine.initial_state) {
                // Store colors for this profile
                manager.set_profile_colors(&item.label, &state.display.bg, &state.display.fg);
            }
        }
        
        manager
    }
    
    // Create a unique key for a profile+button
    pub fn make_key(profile: &str, button: &str) -> String {
        format!("{}:{}", profile, button)
    }
    
    // Set colors for a profile
    fn set_profile_colors(&mut self, profile: &str, bg: &str, fg: &str) {
        let bg_key = Self::make_key(profile, "bg");
        let fg_key = Self::make_key(profile, "fg");
        
        self.button_colors.insert(bg_key, bg.to_string());
        self.button_colors.insert(fg_key, fg.to_string());
    }
    
    // Process a button action according to state machine rules
    pub fn press_button(&mut self, profile: &str, action: &str) -> bool {
        // First, extract all the needed data from the config to avoid borrowing conflicts
        let transition_data = {
            // Get the menu configuration
            let config = match &self.menu_config {
                Some(config) => config,
                None => return false, // Can't process without config
            };
            
            // Find the menu item for this profile
            let item_config = match config.menu_items.iter().find(|item| item.label == profile) {
                Some(item) => item,
                None => return false, // Profile not found
            };
            
            // Get current state for this profile
            let current_state = self.profile_states.get(profile)
                .unwrap_or(&item_config.state_machine.initial_state)
                .clone();
            
            // Check if this state has the requested transition
            if let Some(state) = item_config.state_machine.states.get(&current_state) {
                if let Some(next_state) = state.transitions.get(action) {
                    // Look up the next state for its display properties
                    if let Some(new_state) = item_config.state_machine.states.get(next_state) {
                        // Return the data we need for the transition
                        Some((
                            next_state.clone(),
                            new_state.display.bg.clone(),
                            new_state.display.fg.clone()
                        ))
                    } else {
                        None // Next state not defined
                    }
                } else {
                    None // Transition not found
                }
            } else {
                None // Current state not defined
            }
        };
        
        // Now perform the transition if we found valid data
        if let Some((next_state, bg, fg)) = transition_data {
            // Apply the transition with our extracted data
            self.profile_states.insert(profile.to_string(), next_state);
            self.set_profile_colors(profile, &bg, &fg);
            true
        } else {
            false
        }
    }
    
    // Get current state for a profile
    pub fn get_current_state(&self, profile: &str) -> Option<String> {
        self.profile_states.get(profile).cloned()
    }
    
    // Get background color for a profile
    pub fn get_background_color(&self, profile: &str) -> String {
        let key = Self::make_key(profile, "bg");
        self.button_colors.get(&key).cloned().unwrap_or_else(|| "#2E2E2E".to_string())
    }
    
    // Get foreground color for a profile
    pub fn get_foreground_color(&self, profile: &str) -> String {
        let key = Self::make_key(profile, "fg");
        self.button_colors.get(&key).cloned().unwrap_or_else(|| "#D3D3D3".to_string())
    }
    
    // Get color for a button (legacy method for compatibility)
    pub fn get_button_color(&self, profile: &str, _button: &str) -> String {
        // Return the background color for the profile's current state
        self.get_background_color(profile)
    }
    
    // Get available actions for the current state
    pub fn get_available_actions(&self, profile: &str) -> Vec<String> {
        let mut actions = Vec::new();
        
        // Get configuration and current state
        if let Some(config) = &self.menu_config {
            if let Some(item) = config.menu_items.iter().find(|item| item.label == profile) {
                if let Some(current_state) = self.get_current_state(profile) {
                    if let Some(state) = item.state_machine.states.get(&current_state) {
                        // Add all transitions from current state
                        for action in state.transitions.keys() {
                            actions.push(action.clone());
                        }
                    }
                }
            }
        }
        
        actions
    }
}