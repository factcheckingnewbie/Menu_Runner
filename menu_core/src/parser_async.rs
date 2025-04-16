use std::collections::HashMap;
use crate::models::CommandInfo;

// Keep your existing load_menu_async function

// Add these missing functions:
pub async fn group_menu_commands(commands: Vec<CommandInfo>) -> HashMap<String, Vec<CommandInfo>> {
    let mut grouped: HashMap<String, Vec<CommandInfo>> = HashMap::new();
    
    for cmd in commands {
        grouped.entry(cmd.category.clone())
               .or_insert_with(Vec::new)
               .push(cmd);
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
