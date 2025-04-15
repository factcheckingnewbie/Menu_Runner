// menu_core/src/parser.rs
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;

use crate::models::{MenuCommand, GroupedMenuEntry};

/// Loads menu entries from a menu.txt file
pub fn load_menu() -> Vec<MenuCommand> {
    // Check if menu.txt exists
    if !Path::new("menu.txt").exists() {
        println!("ERROR: menu.txt file not found!");
        return Vec::new();
    }
    
    let file = match File::open("menu.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open menu.txt: {}", e);
            return Vec::new();
        }
    };
    
    let reader = BufReader::new(file);
    let mut commands = Vec::new();
    let mut line_number = 0;
    
    for line in reader.lines() {
        line_number += 1;
        match line {
            Ok(line) => {
                if line.trim().is_empty() {
                    println!("Line {}: Empty line, skipping", line_number);
                    continue;
                }
                
                // Parse quoted strings
                let mut parts: Vec<String> = Vec::new();
                let mut current_part = String::new();
                let mut in_quotes = false;
                
                for c in line.chars() {
                    if c == '"' {
                        // Toggle quote state
                        in_quotes = !in_quotes;
                        
                        // If we're ending a quoted section, save it
                        if !in_quotes && !current_part.is_empty() {
                            parts.push(current_part.clone());
                            current_part.clear();
                        }
                    } else if in_quotes {
                        // Inside quotes, accumulate characters
                        current_part.push(c);
                    }
                    // Skip spaces outside quotes
                }
                
                // Clean up trailing quote in case it exists
                if in_quotes && !current_part.is_empty() {
                    current_part = current_part.trim_end_matches('"').to_string();
                    parts.push(current_part);
                }
                
                if parts.len() >= 2 {
                    println!("Line {}: Added menu item \"{}\" with command \"{}\"", 
                             line_number, parts[0], parts[1]);
                    commands.push(MenuCommand {
                        name: parts[0].clone(),
                        command: parts[1].clone(),
                    });
                } else {
                    println!("Line {}: Invalid format (expected two quoted sections): {}", 
                             line_number, line);
                }
            },
            Err(e) => println!("Error reading line {}: {}", line_number, e),
        }
    }
    
    println!("Loaded {} menu items from menu.txt", commands.len());
    commands
}

/// Extract program name and path from command string
pub fn extract_command_info(command: &str) -> (String, String, String) {
    // Extract action from command (e.g., 'start', 'freeze', etc.)
    let action = if let Some(action_start) = command.find('\'') {
        if let Some(action_end) = command[action_start + 1..].find('\'') {
            command[action_start + 1..action_start + 1 + action_end].to_string()
        } else {
            "unknown".to_string()
        }
    } else {
        "unknown".to_string()
    };
    
    // Extract program name (assuming it's after the action)
    let program_parts: Vec<&str> = command.split_whitespace().collect();
    let program = if program_parts.len() > 2 {
        program_parts[2].to_string()
    } else {
        "unknown".to_string()
    };
    
    // Extract path (last segment after final slash)
    let path = if let Some(last_slash) = command.rfind('/') {
        command[last_slash + 1..].trim_end_matches('"').to_string()
    } else {
        "unknown".to_string()
    };
    
    (action, program, path)
}

/// Group menu commands by program and path
pub fn group_menu_commands(commands: &[MenuCommand]) -> Vec<GroupedMenuEntry> {
    let mut groups: HashMap<String, GroupedMenuEntry> = HashMap::new();
    
    for cmd in commands {
        let (action, program, path) = extract_command_info(&cmd.command);
        let key = format!("{}:{}", program, path);
        
        // If group exists, add action and command to it
        if let Some(group) = groups.get_mut(&key) {
            group.actions.push(action);
            group.commands.push(cmd.command.clone());
        } else {
            // Create new group
            groups.insert(key, GroupedMenuEntry {
                program,
                path_name: path,
                actions: vec![action],
                commands: vec![cmd.command.clone()],
            });
        }
    }
    
    groups.into_values().collect()
}
