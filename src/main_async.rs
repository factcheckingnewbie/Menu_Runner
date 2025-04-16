use std::process::Command;
use std::rc::Rc;
use std::sync::Arc;
use slint::{ModelRc, VecModel, Weak, SharedString};
use tokio::runtime::Runtime;
use tokio::process::Command as TokioCommand;
use tokio::task;

// Create a tokio runtime for async operations
fn main() {
    // Create the runtime
    let rt = Runtime::new().unwrap();
    
    // Enter the runtime context
    rt.block_on(async {
        // Load menu asynchronously
        let commands = Menu_Runner_core::load_menu_async().await;
        
        if commands.is_empty() {
            println!("No valid menu items found. Please check your configs/menu.txt format.");
            return;
        }
        
        // Group commands by path
        let grouped_commands = Menu_Runner_core::group_menu_commands(&commands);
        println!("Created {} menu groups", grouped_commands.len());
        
        // Create UI and set up handlers as before
        let main_window = MainWindow::new().unwrap();
        
        // Convert to menu items as before...
        let window_weak = main_window.as_weak();
        
        // Set up async command handler
        main_window.on_run_command(move |command_text| {
            let command_str = command_text.to_string();
            let window_clone = window_weak.clone();
            
            // Spawn the command in the runtime
            rt.spawn(async move {
                run_command_async(&command_str, window_clone).await;
            });
        });
        
        println!("Starting UI...");
        
        // Run the UI loop (this will block the current async task)
        main_window.run().unwrap();
    });
}

async fn run_command_async(command_text: &str, window: Weak<MainWindow>) {
    // Set status to indicate we're starting the command
    let command_for_status = command_text.to_string();
    window.upgrade_in_event_loop(move |handle| {
        handle.set_status_text(format!("Running: {}", command_for_status).into());
    }).await.expect("Failed to upgrade window handle");
    
    // Execute the command asynchronously
    let output = TokioCommand::new("sh")
        .arg("-c")
        .arg(command_text)
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
            
            // Update UI with status
            let status_message = if status.success() {
                format!("Command completed successfully")
            } else {
                format!("Command failed with status {}", status)
            };
            
            window.upgrade_in_event_loop(move |handle| {
                handle.set_status_text(status_message.into());
            }).await.ok();
            
            // Clear the status after a few seconds
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            
            window.upgrade_in_event_loop(|handle| {
                handle.set_status_text("".into());
            }).await.ok();
        },
        Err(e) => {
            let error_message = format!("Failed to execute command: {}", e);
            println!("{}", error_message);
            
            window.upgrade_in_event_loop(move |handle| {
                handle.set_status_text(error_message.into());
            }).await.ok();
        }
    }
}
