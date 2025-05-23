// Import necessary Rust and external crates
use std::rc::Rc;
use std::process::Command;
use std::sync::Mutex;
use std::collections::HashMap;

// Include the Slint modules defined in your .slint files
slint::include_modules!();
use slint::{ModelRc, VecModel, SharedString};
use tokio::runtime::Runtime;

// Import the core types from our menu_core library
use Menu_Runner_core::create_slint_menu_entries;

fn main() {
    // Create the runtime with all features enabled
    let rt = Runtime::new().unwrap();

    // Enter the runtime context
    rt.block_on(async {
        println!("Starting async menu loader...");

        // Load menu and button manager - properly handle the Result type
        let (commands, button_manager) = match Menu_Runner_core::load_menu_with_button_manager().await {
            Ok((cmds, manager)) => (cmds, manager),
            Err(e) => {
                println!("Error loading menu with button manager: {}", e);
                (Vec::new(), Menu_Runner_core::ButtonManager::new())
            }
        };
        
        let button_manager = Rc::new(Mutex::new(button_manager));
        
        if commands.is_empty() {
           println!("No valid menu items found. Please check your configs/menu_config_color.yaml format.");
           return;
        }

        println!("Successfully loaded {} menu items", commands.len());

        // Create the Slint menu entries from the command info
        let slint_entries = create_slint_menu_entries(&commands);
        println!("Created {} menu entries for the UI", slint_entries.len());

        // Keep track of all possible actions per profile
        let mut all_actions_by_profile: HashMap<String, Vec<String>> = HashMap::new();
        for entry in &slint_entries {
            all_actions_by_profile.insert(entry.label.clone(), entry.actions.clone());
        }

        // Create the main window from your Slint UI definition
        let main_window = MainWindow::new().unwrap();
        
        // Function to build menu model with only available actions
        let build_menu_model = move |button_manager: &std::sync::MutexGuard<'_, Menu_Runner_core::ButtonManager>| {
            let menu_entries: Vec<MenuEntry> = slint_entries.iter().map(|entry| {
                // Get only available actions for current state
                let available_actions = button_manager.get_available_actions(&entry.label);
                
                // Make sure we only include actions that exist in our UI
                let filtered_actions: Vec<String> = all_actions_by_profile.get(&entry.label)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .filter(|a| available_actions.contains(a))
                    .cloned()
                    .collect();
                
                // Convert to SharedString for Slint
                let actions_vec: Vec<SharedString> = filtered_actions.iter()
                    .map(|a| a.clone().into())
                    .collect();
                
                // Create a VecModel from the actions and convert it to ModelRc
                let actions_model = Rc::new(VecModel::from(actions_vec));
                
                MenuEntry {
                    label: entry.label.clone().into(),
                    actions: ModelRc::from(actions_model),
                    command_template: entry.command_template.clone().into(),
                }
            }).collect();
            
            Rc::new(VecModel::from(menu_entries))
        };
        
        // Initial menu model
        let menu_model = build_menu_model(&button_manager.lock().unwrap());
        main_window.set_menu_items(ModelRc::from(menu_model.clone()));
        
        // Set up button color provider callback - simplified to avoid unnecessary calculations
        main_window.on_get_button_color(move |_profile, _action| {
            // Since UI doesn't use colors anymore, return a simple default
            // This maintains compatibility with the callback signature
            "#007BFF".into()
        });
        
        // Set up command handler for when action buttons are clicked
        let button_manager_click = button_manager.clone();
        let weak_window = main_window.as_weak();
        main_window.on_run_command(move |command_template, action| {
            // Extract profile from command template
            let profile_name: String = {
                // Create a variable for the string to extend its lifetime
                let cmd_str = command_template.to_string();
                let parts: Vec<&str> = cmd_str.split('/').collect();
                
                if let Some(last) = parts.last() {
                    // Extract profile from filename pattern
                    if let Some(filename) = last.split('.').nth(1) {
                        filename.to_string()
                    } else {
                        // Extract label from path as fallback
                        parts.iter()
                            .rev()
                            .skip(1)
                            .next()
                            .unwrap_or(&"unknown")
                            .to_string()
                    }
                } else {
                    "unknown".to_string()
                }
            };
             
            println!("Executing action '{}' for profile '{}'", action, profile_name);
            
            // Update button visual state using state machine
            {
                let mut manager = button_manager_click.lock().unwrap();
                manager.press_button(&profile_name, &action.to_string());
            }
                 
            // Replace the action placeholder - FIXED: Replace {ACTION} instead of just ACTION
            let mut command_str = command_template.to_string().replace("{ACTION}", &action.to_string());

            // Remove any quotes that would be interpreted literally by the shell
            command_str = command_str.replace("\"./target/debug/Menu_Runner_system\"", "./target/debug/Menu_Runner_system");
            command_str = command_str.replace("\"firefox", "firefox");
            command_str = command_str.trim_end_matches('"').to_string();

            println!("Running command: {}", command_str);

            // Execute the command
            let output_result = Command::new("sh")
                .arg("-c")
                .arg(&command_str)
                .output();

            match output_result {
                Ok(output) => {
                    let status = output.status;
                    println!("Command completed with status: {}", status);
                },
                Err(e) => {
                    println!("Failed to execute command: {}", e);
                }
            }
            
            // Rebuild the menu model with updated states
            let new_menu_model = build_menu_model(&button_manager_click.lock().unwrap());
            
            // Update the UI with the new menu model on state change
            if let Some(window) = weak_window.upgrade() {
                window.set_menu_items(ModelRc::from(new_menu_model));
            }
        });

        println!("Starting UI...");
        main_window.run().unwrap();
    });
}
