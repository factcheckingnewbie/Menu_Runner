// menu_core/src/lib.rs
// Export all public items from this crate
pub mod models;
pub mod parser_async;

pub use models::{CommandInfo, GroupedMenuEntry, SlintMenuEntry};
pub use models::{ButtonManager, MenuConfig};
pub use parser_async::{
    load_menu_async, load_menu_json_async, load_menu_yaml_async,
    load_menu_config_color, load_menu_with_button_manager,
    create_slint_menu_entries, build_grouped_entries, MenuError
};