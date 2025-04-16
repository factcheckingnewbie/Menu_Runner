// menu_core/src/parser_async.rs
use std::fs;
use std::path::Path;
use crate::models::MenuCommand;
use tokio::fs as tokio_fs;

/// Load menu commands from the default menu file location asynchronously
pub async fn load_menu_async() -> Vec<MenuCommand> {
    // First try the new format
    let new_format_path = Path::new("configs/future_menu.txt");
    if new_format_path.exists() {
        if let Ok(content) = tokio_fs::read_to_string(new_format_path).await {
            let commands = parse_new_menu_format(&content);
            if !commands.is_empty() {
                return commands;
            }
        }
    }
    
    // Fall back to the traditional format if new format failed
    let traditional_path = Path::new("configs/menu.txt");
    if let Ok(content) = tokio_fs::read_to_string(traditional_path).await {
        return parse_traditional_menu(&content);
    }
    
    Vec::new()
}

// Other parsing functions remain the same as they're not I/O bound
// ...
