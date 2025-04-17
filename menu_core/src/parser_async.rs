use std::collections::HashMap;
use crate::models::CommandInfo;
use std::path::Path;
use tokio::fs as tokio_fs;

pub async fn load_menu_async() -> Vec<crate::models::CommandInfo> {
    let menu_path = Path::new("configs/future_menu.txt");
    
    if !menu_path.exists() {
        println!("Warning: configs/future_menu.txt not found");
        return Vec::new();
    }
    
    match tokio_fs::read_to_string(menu_path).await {
        Ok(content) => parse_future_menu_format(&content),
        Err(e) => {
            println!("Error reading future_menu.txt: {}", e);
            Vec::new()
        }
    }
}

fn parse_future_menu_format(content: &str) -> Vec<CommandInfo> {
    let mut commands = Vec::new();
    let mut current_label = String::new();
    let mut current_actions = Vec::new();
    let mut current_command = String::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        
        if let Some(label) = trimmed.strip_prefix("Label:") {
            // Process previous entry if complete
            if !current_label.is_empty() && !current_actions.is_empty() && !current_command.is_empty() {
                for action in &current_actions {
                    commands.push(CommandInfo {
                        name: format!("{} {}", current_label, action),
                        command: current_command.replace("<Action>", action),
                        description: format!("{} operation for {}", action, current_label),
                        category: current_label.clone(),
                    });
                }
            }
            
            // Start a new entry
            current_label = label.trim().trim_matches('"').to_string();
            current_actions.clear();
            current_command.clear();
        } else if let Some(actions) = trimmed.strip_prefix("Actions:") {
            current_actions = actions
                .split_whitespace()
                .map(|s| s.trim_matches('"').to_string())
                .collect();
        } else if let Some(cmd) = trimmed.strip_prefix("Command:") {
            current_command = cmd.trim().to_string();
        }
    }
    
    // Process the last entry
    if !current_label.is_empty() && !current_actions.is_empty() && !current_command.is_empty() {
        for action in &current_actions {
            commands.push(CommandInfo {
                name: format!("{} {}", current_label, action),
                command: current_command.replace("<Action>", action),
                description: format!("{} operation for {}", action, current_label),
                category: current_label.clone(),
            });
        }
    }
    
    commands
}

// Helper function for grouping commands by category
pub fn group_menu_commands(commands: &[CommandInfo]) -> HashMap<String, Vec<CommandInfo>> {
    let mut grouped: HashMap<String, Vec<CommandInfo>> = HashMap::new();
    
    for cmd in commands {
        grouped
            .entry(cmd.category.clone())
            .or_insert_with(Vec::new)
            .push(cmd.clone());
    }
    
    grouped
}

// This function can be kept for consistency with existing code, but isn't needed
// for the future_menu.txt format
pub fn extract_command_info(line: &str) -> Option<CommandInfo> {
    None // Not used with the future_menu.txt format
}
