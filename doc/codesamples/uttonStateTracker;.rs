use std::collections::HashMap;
use regex::Regex;

// Add this to your parser module
pub struct ButtonStateTracker {
    button_states: HashMap<String, bool>,
    button_affects: HashMap<String, Vec<String>>,
    button_colors: HashMap<String, String>,
}

impl ButtonStateTracker {
    pub fn new() -> Self {
        Self {
            button_states: HashMap::new(),
            button_affects: HashMap::new(),
            button_colors: HashMap::new(),
        }
    }

    // Initialize from parsed menu entries
    pub fn initialize_from_menu(&mut self, menu_items: &[MenuEntry]) {
        for item in menu_items {
            let profile = &item.label;
            
            for action in &item.actions {
                let key = format!("{}:{}", profile, action);
                
                // Default values
                self.button_states.insert(key.clone(), false);
                self.button_colors.insert(key.clone(), "#007BFF".to_string());
                
                // Extract related buttons from action name patterns
                // For example: freeze->unfreeze, start->stop
                self.parse_button_relationships(profile, action);
            }
        }
    }
    
    fn parse_button_relationships(&mut self, profile: &str, action: &str) {
        let key = format!("{}:{}", profile, action);
        let mut affects = Vec::new();
        
        // Simple opposites detection
        if action.contains("freeze") {
            affects.push(format!("{}:{}", profile, action.replace("freeze", "unfreeze")));
        } else if action.contains("unfreeze") {
            affects.push(format!("{}:{}", profile, action.replace("unfreeze", "freeze")));
        } else if action.contains("start") {
            affects.push(format!("{}:{}", profile, action.replace("start", "stop")));
        } else if action.contains("stop") {
            affects.push(format!("{}:{}", profile, action.replace("stop", "start")));
        }
        
        if !affects.is_empty() {
            self.button_affects.insert(key, affects);
        }
    }
    
    pub fn toggle_button(&mut self, profile: &str, action: &str) -> bool {
        let key = format!("{}:{}", profile, action);
        
        // Toggle the state
        let current = self.button_states.get(&key).cloned().unwrap_or(false);
        let new_state = !current;
        self.button_states.insert(key.clone(), new_state);
        
        // Update related buttons
        if let Some(affects) = self.button_affects.get(&key).cloned() {
            for affected_key in affects {
                self.button_states.insert(affected_key, !new_state);
            }
        }
        
        new_state
    }
    
    pub fn is_button_active(&self, profile: &str, action: &str) -> bool {
        let key = format!("{}:{}", profile, action);
        self.button_states.get(&key).cloned().unwrap_or(false)
    }
    
    pub fn get_button_color(&self, profile: &str, action: &str) -> String {
        let key = format!("{}:{}", profile, action);
        let is_active = self.is_button_active(profile, action);
        
        if is_active {
            "#3d8f46".to_string() // Green for active
        } else {
            "#007BFF".to_string() // Blue for inactive
        }
    }
}

// Modify your existing load_menu function to create the tracker
pub async fn load_menu_with_button_tracker(filename: &str) -> Result<(Vec<MenuEntry>, ButtonStateTracker), Box<dyn std::error::Error>> {
    let menu_items = load_menu(filename).await?;
    
    let mut tracker = ButtonStateTracker::new();
    tracker.initialize_from_menu(&menu_items);
    
    Ok((menu_items, tracker))
}