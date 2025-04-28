use std::collections::HashMap;
use std::path::Path;
use std::io;
use std::fmt;
use crate::models::{CommandInfo, GroupedMenuEntry, SlintMenuEntry};
use crate::models::{ButtonManager, MenuConfig};
use tokio::fs as tokio_fs;

// Custom error type for menu loading operations
#[derive(Debug)]
pub enum MenuError {
    IoError(io::Error),
    ParseError(String),
    FileNotFound(String),
}

impl fmt::Display for MenuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuError::IoError(err) => write!(f, "I/O error: {}", err),
            MenuError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            MenuError::FileNotFound(path) => write!(f, "File not found: {}", path),
        }
    }
}

impl From<io::Error> for MenuError {
    fn from(err: io::Error) -> Self {
        MenuError::IoError(err)
    }
}

impl From<serde_json::Error> for MenuError {
    fn from(err: serde_json::Error) -> Self {
        MenuError::ParseError(err.to_string())
    }
}

impl From<serde_yaml::Error> for MenuError {
    fn from(err: serde_yaml::Error) -> Self {
        MenuError::ParseError(err.to_string())
    }
}

impl std::error::Error for MenuError {}

pub async fn load_menu_async() -> Result<Vec<CommandInfo>, MenuError> {
    let menu_path = Path::new("configs/future_menu.txt");

    if !menu_path.exists() {
        return Err(MenuError::FileNotFound("configs/future_menu.txt".to_string()));
    }

    let content = tokio_fs::read_to_string(menu_path).await?;
    Ok(parse_future_menu_format(&content))
}

pub async fn load_menu_json_async() -> Result<Vec<CommandInfo>, MenuError> {
    let menu_path = Path::new("configs/future_menu.json");

    if !menu_path.exists() {
        return Err(MenuError::FileNotFound("configs/future_menu.json".to_string()));
    }

    let content = tokio_fs::read_to_string(menu_path).await?;
    let commands = serde_json::from_str::<Vec<CommandInfo>>(&content)?;
    Ok(commands)
}

fn parse_future_menu_format(content: &str) -> Vec<CommandInfo> {
    let mut result = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        if !line.is_empty() && !line.starts_with('#') {
            if let Some(command_info) = extract_command_info(line) {
                result.push(command_info);
            }
        }
        i += 1;
    }

    result
}

pub async fn load_menu_yaml_async() -> Result<Vec<CommandInfo>, MenuError> {
    let menu_path = Path::new("configs/menu_config.yaml");

    if !menu_path.exists() {
        return Err(MenuError::FileNotFound("configs/menu_config.yaml".to_string()));
    }

    let content = tokio_fs::read_to_string(menu_path).await?;
    let commands = serde_yaml::from_str::<Vec<CommandInfo>>(&content)?;
    Ok(commands)
}

pub async fn load_menu_config_color() -> Result<MenuConfig, MenuError> {
    let config_path = Path::new("configs/menu_config_color.yaml");

    if !config_path.exists() {
        return Err(MenuError::FileNotFound("configs/menu_config_color.yaml".to_string()));
    }

    let content = tokio_fs::read_to_string(config_path).await?;
    let config = serde_yaml::from_str::<MenuConfig>(&content)?;
    println!("Successfully loaded menu config with {} items", config.menu_items.len());
    Ok(config)
}

pub fn group_menu_commands(commands: &[CommandInfo]) -> HashMap<String, Vec<CommandInfo>> {
    let mut grouped = HashMap::new();

    for cmd in commands {
        let entry = grouped.entry(cmd.category.clone()).or_insert_with(Vec::new);
        entry.push(cmd.clone());
    }

    grouped
}

pub fn build_grouped_entries(commands: &[CommandInfo]) -> Vec<GroupedMenuEntry> {
    let mut result = Vec::new();
    let grouped = group_menu_commands(commands);

    for (category, cmds) in grouped {
        if !cmds.is_empty() {
            let path_name = if let Some(first_cmd) = cmds.first() {
                let parts: Vec<&str> = first_cmd.command.split(" ").collect();
                if parts.len() >= 3 {
                    parts[2].to_string()
                } else {
                    category.clone()
                }
            } else {
                category.clone()
            };

            let actions = cmds.iter()
                .map(|cmd| {
                    let parts: Vec<&str> = cmd.name.split(' ').collect();
                    if parts.len() > 1 {
                        parts[parts.len() - 1].to_string()
                    } else {
                        "default".to_string()
                    }
                })
                .collect();

            let commands = cmds.iter()
                .map(|cmd| cmd.command.clone())
                .collect();

            result.push(GroupedMenuEntry {
                program: category.clone(),
                path_name,
                actions,
                commands,
            });
        }
    }

    result
}

pub async fn load_menu_with_button_manager() -> Result<(Vec<CommandInfo>, ButtonManager), MenuError> {
    // Load state machine data from menu_config_color.yaml
    let config = load_menu_config_color().await?;
    
    // Extract command info from the MenuConfig
    let mut commands = Vec::new();
    
    for item in &config.menu_items {
        // Extract available actions from the state machine transitions
        let mut actions = Vec::new();
        
        if let Some(default_state) = item.state_machine.states.get(&item.state_machine.initial_state) {
            for action in default_state.transitions.keys() {
                actions.push(action.clone());
            }
        }
        
        // Create CommandInfo objects from the actions
        for action in &actions {
            commands.push(CommandInfo {
                name: format!("{} {}", item.label, action),
                command: item.command_template.replace("{ACTION}", action),
                description: format!("{} operation for {}", action, item.label),
                category: item.label.clone(),
            });
        }
    }
    
    println!("Extracted {} commands from menu_config_color.yaml", commands.len());
    
    // Create button manager from the same config
    let button_manager = ButtonManager::from_menu_config(config);
    
    Ok((commands, button_manager))
}

pub fn extract_command_info(_line: &str) -> Option<CommandInfo> {
    None // Not used with the new menu formats
}

pub fn create_slint_menu_entries(commands: &[CommandInfo]) -> Vec<SlintMenuEntry> {
    let mut result = Vec::new();
    let grouped = group_menu_commands(commands);
    
    for (category, cmds) in &grouped {
        if !cmds.is_empty() {
            let command_template = if let Some(first_cmd) = cmds.first() {
                let parts: Vec<&str> = first_cmd.command.split(" ").collect();
                if parts.len() >= 3 {
                    format!("{} {{ACTION}} {}", parts[0], parts[2..].join(" "))
                } else {
                    first_cmd.command.clone()
                }
            } else {
                String::new()
            };
            
            let mut actions = Vec::new();
            for cmd in cmds {
                let parts: Vec<&str> = cmd.name.split(' ').collect();
                if parts.len() > 1 {
                    let action = parts[parts.len() - 1].to_string();
                    if !actions.contains(&action) {
                        actions.push(action);
                    }
                }
            }
            
            result.push(SlintMenuEntry {
                label: category.clone(),
                actions,
                command_template,
            });
        }
    }
    
    result
}

#[deprecated(
    since = "0.2.0",
    note = "Use ButtonManager::from_menu_config() with data from menu_config_color.yaml instead"
)]
pub fn initialize_button_manager(profiles: &[CommandInfo]) -> ButtonManager {
    let mut manager = ButtonManager::new();
    
    let grouped = group_menu_commands(profiles);
    
    for (profile_name, cmds) in grouped {
        let mut actions = Vec::new();
        for cmd in &cmds {
            let parts: Vec<&str> = cmd.name.split(' ').collect();
            if parts.len() > 1 {
                let action = parts[parts.len() - 1];
                if !actions.contains(&action) {
                    actions.push(action);
                }
            }
        }
        
        if actions.contains(&"freeze") && actions.contains(&"unfreeze") {
            let freeze_key = ButtonManager::make_key(&profile_name, "freeze");
            let unfreeze_key = ButtonManager::make_key(&profile_name, "unfreeze");
            
            manager.button_colors.insert(freeze_key, "#007BFF".to_string());
            manager.button_colors.insert(unfreeze_key, "#007BFF".to_string());
        }
        
        if actions.contains(&"kill") && actions.contains(&"start") {
            let kill_key = ButtonManager::make_key(&profile_name, "kill");
            let start_key = ButtonManager::make_key(&profile_name, "start");
            
            manager.button_colors.insert(kill_key, "#007BFF".to_string());
            manager.button_colors.insert(start_key, "#007BFF".to_string());
        }
    }
    
    manager
}
