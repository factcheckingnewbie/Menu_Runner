// menu_core/src/lib.rs
// Export all public items from this crate
pub mod parser_async;
pub mod models;

// Public exports
pub use models::{MenuCommand, GroupedMenuEntry, CommandInfo, SlintMenuEntry};
pub use parser_async::{load_menu_async, load_menu_json_async, extract_command_info, group_menu_commands, build_grouped_entries, create_slint_menu_entries};
