/// menu_core/src/models.rs
/// Represents a single menu command entry
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Basic Command Information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInfo {
    pub name: String,
    pub command: String,
    pub description: String,
    pub category: String,
}

// Structure for grouped menu entries
pub struct GroupedMenuEntry {
    pub program: String,
    pub path_name: String,
    pub actions: Vec<String>,
    pub commands: Vec<String>,
}

// Structure for Slint menu entries
#[derive(Debug, Clone)]
pub struct SlintMenuEntry {
    pub label: String,
    pub actions: Vec<String>,
    pub command_template: String,
}

// State in the state machine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub transitions: HashMap<String, String>,
    #[serde(default)]  // Make style field optional with default empty HashMap
    pub style: HashMap<String, String>,
}

impl State {
    // Helper method to get color with default
    pub fn get_color(&self) -> String {
        self.style.get("color").cloned().unwrap_or_else(|| "#007BFF".to_string())
    }
}

// State machine definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachine {
    pub initial_state: String,
    pub states: HashMap<String, State>,
}

// Menu item configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItemConfig {
    pub label: String,
    pub command_template: String,
    pub state_machine: StateMachine,
}

// Overall menu configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuConfig {
    pub menu_items: Vec<MenuItemConfig>,
}

// Button manager that tracks button states
pub struct ButtonManager {
    pub button_states: HashMap<String, String>,
    pub button_colors: HashMap<String, String>,
    pub menu_config: Option<MenuConfig>,
}

impl ButtonManager {
    pub fn new() -> Self {
        ButtonManager {
            button_states: HashMap::new(),
            button_colors: HashMap::new(),
            menu_config: None,
        }
    }

    pub fn make_key(profile: &str, action: &str) -> String {
        format!("{}:{}", profile, action)
    }

    pub fn from_menu_config(config: MenuConfig) -> Self {
        let mut manager = ButtonManager::new();
        manager.menu_config = Some(config.clone());
        
        // Initialize button states based on the menu config
        for item in &config.menu_items {
            for (_state_name, state) in &item.state_machine.states {
                for (action, _next_state) in &state.transitions {
                    let key = Self::make_key(&item.label, action);
                    manager.button_states.insert(key.clone(), item.state_machine.initial_state.clone());
                    
                    // Set default color if defined in the state style
                    if let Some(initial_state) = item.state_machine.states.get(&item.state_machine.initial_state) {
                        if let Some(color) = initial_state.style.get("color") {
                            manager.button_colors.insert(key.clone(), color.clone());
                        } else {
                            manager.button_colors.insert(key.clone(), "#007BFF".to_string());
                        }
                    }
                }
            }
        }
        
        manager
    }

    pub fn press_button(&mut self, profile: &str, action: &str) {
        let key = Self::make_key(profile, action);
        
        if let Some(config) = &self.menu_config {
            for item in &config.menu_items {
                if item.label == profile {
                    // Get the current state for this button
                    let current_state = self.button_states.get(&key).unwrap_or(&item.state_machine.initial_state).clone();
                    
                    // Find the transition for this action
                    if let Some(state) = item.state_machine.states.get(&current_state) {
                        if let Some(next_state) = state.transitions.get(action) {
                            // Update the button state
                            println!("Button state changed: {} -> {}", current_state, next_state);
                            self.button_states.insert(key.clone(), next_state.clone());
                            
                            // Update button color if specified in the state style
                            if let Some(next_state_def) = item.state_machine.states.get(next_state) {
                                if let Some(color) = next_state_def.style.get("color") {
                                    self.button_colors.insert(key.clone(), color.clone());
                                }
                            }
                        }
                    }
                    break;
                }
            }
        }
    }

    // Corrected get_action_color method that uses existing fields
    pub fn get_action_color(&self, profile: &str, action: &str) -> String {
        let key = Self::make_key(profile, action);
        
        // Get the current state name for this profile+action using button_states
        let state_name = self.button_states
            .get(&key)
            .cloned()
            .unwrap_or_else(|| "default".to_string());
        
        // Check if we already have a color stored
        if let Some(color) = self.button_colors.get(&key) {
            return color.clone();
        }
        
        // Try to get color from state machine if configuration exists
        if let Some(config) = &self.menu_config {
            // Find the menu item for this profile
            for item in &config.menu_items {
                if item.label == profile {
                    // If we found the profile, look for the current state
                    if let Some(state) = item.state_machine.states.get(&state_name) {
                        return state.get_color();
                    }
                }
            }
        }
        
        // Default color when nothing is found
        "#007BFF".to_string()
    }
    
    // Dummy background color method - for future use
    pub fn get_background_color(&self, _profile: &str) -> String {
        // This is a placeholder that returns a constant now
        // but can be expanded later to use actual background colors
        "#FFFFFF".to_string()
    }
    
    // Future method for extended color scheme support - stub for now
    pub fn get_extended_colors(&self, _profile: &str, _action: &str) -> HashMap<String, String> {
        // This returns a minimal set of colors that could be extended later
        let mut colors = HashMap::new();
        colors.insert("primary".to_string(), "#007BFF".to_string());
        colors.insert("hover".to_string(), "#0069D9".to_string());
        colors.insert("pressed".to_string(), "#0062CC".to_string());
        colors
    }

    // Add this new method
    pub fn get_available_actions(&self, profile: &str) -> Vec<String> {
        if let Some(config) = &self.menu_config {
            // Find the menu item for this profile
            for item in &config.menu_items {
                if item.label == profile {
                    // First find the current state for any button in this profile
                    let mut current_state = item.state_machine.initial_state.clone();
                    
                    // Look for any button from this profile to get its current state
                    for (key, state) in &self.button_states {
                        if key.starts_with(&format!("{}:", profile)) {
                            current_state = state.clone();
                            break;
                        }
                    }
                    
                    // Now get transitions available from this state
                    if let Some(state) = item.state_machine.states.get(&current_state) {
                        return state.transitions.keys().cloned().collect();
                    }
                }
            }
        }
        
        Vec::new() // No transitions found
    }
}