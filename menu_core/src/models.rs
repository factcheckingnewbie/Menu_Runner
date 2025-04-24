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

/// Button state manager to track button visual states
#[derive(Clone)]
pub struct ButtonManager {
    // Maps "profile:button" -> state (true = down/active)
    pub button_states: HashMap<String, bool>,
    
    // Maps "profile:button" -> list of affected buttons
    pub button_affects: HashMap<String, Vec<String>>,
    
    // Cache of button colors
    pub button_colors: HashMap<String, String>,
}

impl ButtonManager {
    pub fn new() -> Self {
        ButtonManager {
            button_states: HashMap::new(),
            button_affects: HashMap::new(),
            button_colors: HashMap::new(),
        }
    }
    
    // Create a unique key for a profile+button
    pub fn make_key(profile: &str, button: &str) -> String {
        format!("{}:{}", profile, button)
    }
    
    // Updates the state of a button and its affected buttons
    pub fn press_button(&mut self, profile: &str, button: &str) {
        let button_key = Self::make_key(profile, button);
        
        // Toggle this button's state
        let current_state = self.button_states.get(&button_key).cloned().unwrap_or(false);
        self.button_states.insert(button_key.clone(), !current_state);
        
        // Update visual state for affected buttons
        if let Some(affected_buttons) = self.button_affects.get(&button_key) {
            for affected in affected_buttons {
                // Update the color cache
                if !current_state {
                    self.button_colors.insert(affected.clone(), "#3d8f46".to_string()); // Green for active
                } else {
                    self.button_colors.insert(affected.clone(), "#007BFF".to_string()); // Blue for normal
                }
            }
        }
    }
    
    // Get color for a button
    pub fn get_button_color(&self, profile: &str, button: &str) -> String {
        let key = Self::make_key(profile, button);
        self.button_colors.get(&key).cloned().unwrap_or_else(|| "#007BFF".to_string()) // Default blue
    }
}