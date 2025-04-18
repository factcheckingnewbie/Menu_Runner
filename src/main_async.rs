// Import necessary Rust and external crates
use std::rc::Rc;
// Include the Slint modules defined in your .slint files
slint::include_modules!();
use slint::{ModelRc, VecModel, Model, SharedString};
use tokio::runtime::Runtime;
use tokio::process::Command as TokioCommand;
use tokio::task;

// Import the core types from our menu_core library
use Menu_Runner_core::{CommandInfo, GroupedMenuEntry};

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
        
        // Group commands by category
        let grouped_commands = Menu_Runner_core::group_menu_commands(&commands);
        println!("Created {} menu groups", grouped_commands.len());
        
        // Build display entries from groupings
        let grouped_entries = Menu_Runner_core::build_grouped_entries(&commands);
        println!("Built {} grouped entries for hierarchical display", grouped_entries.len());
        
        // Convert CommandInfo objects to MenuEntry structs for the Slint UI
        let menu_entries: Vec<MenuEntry> = commands.iter().enumerate().map(|(i, cmd)| {
            MenuEntry {
                number: i.to_string().into(), // Slint requires SharedString
                name: cmd.name.clone().into(),
                command: cmd.command.clone().into(),
            }
        }).collect();
        
        // Create the Slint model that will hold our menu entries
        let menu_model = Rc::new(VecModel::from(menu_entries));
        
        // Create the main window from your Slint UI definition
        let main_window = MainWindow::new().unwrap();
        
        // Connect the menu_items property in your Slint UI to our model
        main_window.set_menu_items(ModelRc::from(menu_model.clone()));
        
        // Set up command handler for when menu items are clicked
        main_window.on_run_command(move |index| {
            // Get the menu entry at the selected index
            if let Some(entry) = menu_model.row_data(index as usize) {
                let command_str = entry.command.to_string();
                println!("Running command asynchronously: {}", command_str);
                
                // Spawn a new tokio task to execute the command asynchronously
                task::spawn(async move {
                    println!("Executing in async task: {}", command_str);
                    
                    // Execute the command asynchronously using shell
                    let output = TokioCommand::new("sh")
                        .arg("-c")
                        .arg(&command_str)
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
            } else {
                println!("No command found at index {}", index);
            }
        });
        
        println!("Starting UI...");
        
        // Run the UI loop (this blocks the current thread until UI is closed)
        main_window.run().unwrap();
    });
}
