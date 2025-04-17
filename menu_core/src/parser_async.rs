use std::collections::HashMap;
use crate::models::CommandInfo;
use std::path::Path;
use tokio::fs as tokio_fs;
 
// Keep your existing load_menu_async function


// menu_core/src/lib.rs
// Export all public items from this crate

// Public exports
pub use models::{MenuCommand, GroupedMenuEntry, CommandInfo};
pub use parser_async::{extract_command_info}; // Removed load_menu_async

// Load menu function
pub async fn load_menu_async() -> Vec<models::CommandInfo> {
    // Implementation of load_menu_async that was previously in main_async.rs
   
    // First try the new format with extended information
    let new_format_path = Path::new("configs/future_menu.txt");
    if new_format_path.exists() {
        if let Ok(content) = tokio_fs::read_to_string(new_format_path).await {
            let mut commands = Vec::new();
            
            for line in content.lines() {
                if line.trim().is_empty() || line.trim().starts_with('#') {
                    continue; // Skip empty lines and comments
                }
                
                if let Some(cmd_info) = parser_async::extract_command_info(line).await {
                    commands.push(cmd_info);
                }
            }
            
            if !commands.is_empty() {
                return commands;
            }
        }
    }
    
    // Fall back to the traditional format if new format failed
    let traditional_path = Path::new("configs/menu.txt");
    if let Ok(content) = tokio_fs::read_to_string(traditional_path).await {
        let mut commands = Vec::new();
        
        for line in content.lines() {
            if line.trim().is_empty() || line.trim().starts_with('#') {
                continue; // Skip empty lines and comments
            }
            
            // Traditional format only has name and command
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 2 {
                commands.push(models::CommandInfo {
                    name: parts[0].trim().to_string(),
                    command: parts[1].trim().to_string(),
                    description: "".to_string(), // No description in traditional format
                    category: "Uncategorized".to_string(), // Default category
                });
            }
        }
        
        return commands;
    }
    
    Vec::new()
}

// Add the group_menu_commands function at the module level
pub async fn group_menu_commands(commands: &[models::CommandInfo]) -> std::collections::HashMap<String, Vec<models::CommandInfo>> {
    let mut grouped = std::collections::HashMap::new();
    
    for cmd in commands {
        grouped.entry(cmd.category.clone())
               .or_insert_with(Vec::new)
               .push(cmd.clone());
    }
    
    grouped
}




pub async fn extract_command_info(line: &str) -> Option<CommandInfo> {
    // Parse the line using the exact format from your future_menu.txt
    let parts: Vec<&str> = line.split('|').collect();
    
    // Validate we have enough parts for a valid command
    if parts.len() >= 4 {
        Some(CommandInfo {
            name: parts[0].trim().to_string(),
            command: parts[1].trim().to_string(),
            description: parts[2].trim().to_string(),
            category: parts[3].trim().to_string(),
            // If there are additional fields in your CommandInfo struct,
            // you'll need to initialize them here
        })
    } else {
        None
    }
}
