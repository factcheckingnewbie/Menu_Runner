// Import necessary Rust and external crates
use std::rc::Rc;
// Include the Slint modules defined in your .slint files
slint::include_modules!();
use slint::{ModelRc, VecModel, SharedString};
use tokio::runtime::Runtime;
use tokio::process::Command as TokioCommand;
use tokio::task;

// Import the core types from our menu_core library
use Menu_Runner_core::create_slint_menu_entries;

fn main() {
    // Create the runtime with all features enabled
    let rt = Runtime::new().unwrap();
    
    // Enter the runtime context
    rt.block_on(async {
        println!("Starting async menu loader...");
        
        // Load menu asynchronously from your future_menu.txt file
        let commands = Menu_Runner_core::load_menu_async().await;
        
        if commands.is_empty() {
            println!("No valid menu items found. Please check your configs/future_menu.txt format.");
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
        
        // Set up command handler for when action buttons are clicked
        main_window.on_run_command(move |command_template, action| {
            let command_clone = command_template.to_string().replace("<Action>", &action.to_string());
            println!("Running command asynchronously: {}", command_clone);
            
            // Spawn a new tokio task to execute the command asynchronously
            task::spawn(async move {
                println!("Executing in async task: {}", command_clone);
                
                // Execute the command asynchronously using shell
                let output = TokioCommand::new("sh")
                    .arg("-c")
                    .arg(&command_clone)
                    .output()
                    .await;
                
                match output {
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
        });
        
        println!("Starting UI...");
        
        // Run the UI loop (this blocks the current thread until UI is closed)
        main_window.run().unwrap();
    });
}
