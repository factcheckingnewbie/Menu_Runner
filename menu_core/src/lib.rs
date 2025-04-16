// menu_core/src/lib.rs
// Export all public items from this crate
mod parser_async;
mod models;

// Public exports
pub use models::{MenuCommand, GroupedMenuEntry};
pub use parser_async::{load_menu_async};
