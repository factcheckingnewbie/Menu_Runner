// Import necessary Rust and external crates
use std::rc::Rc;
use std::process::Command;
use std::sync::Mutex;

// Include the Slint modules defined in your .slint files
slint::include_modules!();
use slint::{ModelRc, VecModel, SharedString};
use tokio::runtime::Runtime;
// use tokio::process::Command as TokioCommand;
// Import the core types from our menu_core library
 use Menu_Runner_core::create_slint_menu_entries;
// use Menu_Runner_core::{create_slint_menu_entries, models::ButtonManager};

fn main() {
    // Create the runtime with all features enabled
    let rt = Runtime::new().unwrap();

    // Enter the runtime context
    rt.block_on(async {
        println!("Starting async menu loader...");
// Load menu asynchronously from the JSON menu file (changed from txt)
//        let commands = Menu_Runner_core::load_menu_json_async().await;
//        let commands = Menu_Runner_core::load_menu_yaml_async().await;        
// Load menu and button manager
        let (commands, button_manager) = Menu_Runner_core::load_menu_with_button_manager().await;
        let button_manager = Rc::new(Mutex::new(button_manager));
        if commands.is_empty() {
           // println!("No valid menu items found. Please check your configs/future_menu.json format.");
           println!("No valid menu items found. Please check your configs/menu_config.yaml format.");
           return;
        }

        println!("Successfully loaded {} menu items", commands.len());

        // Create the Slint menu entries from the command info
        let slint_entries = create_slint_menu_entries(&commands);
        println!("Created {} menu entries for the UI", slint_entries.len());

        // Convert SlintMenuEntry objects to the Slint UI format
        let menu_entries: Vec<MenuEntry> = slint_entries.iter().map(|entry| {
            // Convert the Rust types to Slint-compatible types
            let actions_vec: Vec<SharedString> = entry.actions.iter()
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

        // Create the Slint model that will hold our menu entries
        let menu_model = Rc::new(VecModel::from(menu_entries));

        // Create the main window from your Slint UI definition
        let main_window = MainWindow::new().unwrap();

        // Connect the menu_items property in your Slint UI to our model
        main_window.set_menu_items(ModelRc::from(menu_model.clone()));
        
        // Set up button color provider callback
        let button_manager_colors = button_manager.clone();
        main_window.on_get_button_color(move |profile, action| {
            let manager = button_manager_colors.lock().unwrap();
            manager.get_button_color(&profile.to_string(), &action.to_string()).into()
        });
        let button_manager_click = button_manager.clone();


        // Set up command handler for when action buttons are clicked
        main_window.on_run_command(move |command_template, action| {
            // Get the command template and replace the action placeholder
            let mut command_str = command_template.to_string();
             // Extract profile name from command template
            let profile_name = {
                let parts: Vec<&str> = command_str.split('/').collect();
                if let Some(last) = parts.last() {
                    last.split('.').nth(1).unwrap_or("").to_string()
                } else {
                    "".to_string()
                }
            };
             
             // Update button visual state
             {
                 let mut manager = button_manager_click.lock().unwrap();
                 manager.press_button(&profile_name, &action.to_string());
             }
                         
            // Replace the action placeholder
            command_str = command_str.replace("ACTION", &action.to_string());

            // Remove any quotes that would be interpreted literally by the shell
            command_str = command_str.replace("\"./target/debug/Menu_Runner_system\"", "./target/debug/Menu_Runner_system");
            command_str = command_str.replace("\"firefox", "firefox");
            command_str = command_str.trim_end_matches('"').to_string();

            println!("Running command synchronously: {}", command_str);

            // Execute the command synchronously using std::process::Command
            // This avoids the nested block_on issue
            let output_result = Command::new("sh")
                .arg("-c")
                .arg(&command_str)
                .output();

            match output_result {
                Ok(output) => {
                    let status = output.status;
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                    println!("Command completed with status: {}", status);
                    println!("Output: {}", stdout);

                    if !stderr.is_empty() {
                        println!("Errors: {}", stderr);
                    }
                },
                Err(e) => {
                    println!("Failed to execute command: {}", e);
                }
            }
        });

        println!("Starting UI...");
        // Notify UI to refresh button colors if needed
        main_window.invoke_refresh();
        
        // Run the UI loop (this blocks the current thread until UI is closed)
        main_window.run().unwrap();

    });
}
