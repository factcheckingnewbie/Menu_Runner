// menu_core/src/lib.rs
// Export all public items from this crate
mod parser;
mod models;

// Public exports
pub use models::{MenuCommand, GroupedMenuEntry};
pub use parser::{load_menu, group_menu_commands, extract_command_info};
