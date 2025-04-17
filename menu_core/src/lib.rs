// menu_core/src/lib.rs
// Export all public items from this crate
pub mod parser_async;
pub mod models;

// Public exports
pub use models::{MenuCommand, GroupedMenuEntry, CommandInfo};
pub use parser_async::{load_menu_async, extract_command_info, group_menu_commands};
