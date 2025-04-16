/// menu_core/src/models.rs

/// Represents a single menu command entry
pub struct MenuCommand {
    pub name: String,
    pub command: String,
}

/// Represents a grouped menu entry with multiple actions
pub struct GroupedMenuEntry {
    pub program: String,
    pub path_name: String,
    pub actions: Vec<String>,
    pub commands: Vec<String>,
}

/// Represents detailed command information with category and description
pub struct CommandInfo {
    pub name: String,
    pub command: String,
    pub description: String,
    pub category: String,
}
