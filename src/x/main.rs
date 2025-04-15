use std::process::Command;
use std::rc::Rc;
use std::thread;
use slint::{ModelRc, VecModel, Weak, SharedString};

// Import from our new menu core library
use Menu_Runner_core::{MenuCommand, GroupedMenuEntry, load_menu, group_menu_commands};

// Define the UI inline with proper imports
slint::slint! {
    import { VerticalBox, ListView, HorizontalBox, Button } from "std-widgets.slint";

    struct MenuEntry {
        program: string,
        path_name: string,
        actions: [string],
        commands: [string],
    }

    export component MainWindow inherits Window {
        title: "Menu Runner";
        width: 400px;
        height: 500px;
        
        callback run_command(string);
        in property <[MenuEntry]> menu_items;
        in property <string> status_text: "";
        
        VerticalBox {
            Text {
                text: "Menu Runner";
                font-size: 24px;
                horizontal-alignment: center;
            }
            
            Rectangle {
                height: 1px;
                background: #ccc;
                visible: menu_items.length > 0;
            }
            
            Text {
                text: "No menu items found in configs/menu.txt";
                visible: menu_items.length <= 0;
                color: red;
            }
            
            ListView {
                visible: menu_items.length > 0;
                for item in menu_items: VerticalLayout {
                    padding: 10px;
                    spacing: 5px;
                    
                    // Action buttons row
                    HorizontalBox {
                        spacing: 10px;
                        alignment: center;
                        
                        for action[index] in item.actions: Button {
                            text: "[ " + action + " ]";
                            clicked => {
                                run_command(item.commands[index]);
                            }
                        }
                    }
                    
                    // Program and path name
                    Text {
                        text: item.program + ": " + item.path_name;
                        font-size: 16px;
                        horizontal-alignment: center;
                    }
                    
                    Rectangle {
                        height: 1px;
                        background: #eee;
                    }
                }
            }
            
            // Status text
            Text {
                text: status_text;
                color: #008000;
                visible: status_text != "";
                font-size: 14px;
            }
        }
    }
}

fn run_command_async(command_text: String, window: Weak<MainWindow>) {
    // Clone command_text for use in the closure
    let command_for_status = command_text.clone();
    
    thread::spawn(move || {
        // Set status to indicate we're starting the command
        window.upgrade_in_event_loop(move |handle| {
            handle.set_status_text(format!("Running: {}", command_for_status).into());
        }).expect("Failed to upgrade window handle");
        
        // Execute the command in a separate thread
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command_text)
            .output();
        
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
                    
                    // Clear the status after a few seconds
                    let window_weak = handle.as_weak();
                    thread::spawn(move || {
                        thread::sleep(std::time::Duration::from_secs(5));
                        window_weak.upgrade_in_event_loop(|handle| {
                            handle.set_status_text("".into());
                        }).ok();
                    });
                }).ok();
            },
            Err(e) => {
                let error_message = format!("Failed to execute command: {}", e);
                println!("{}", error_message);
                
                window.upgrade_in_event_loop(move |handle| {
                    handle.set_status_text(error_message.into());
                }).ok();
            }
        }
    });
}

fn main() {
    let commands = load_menu();
    
    if commands.is_empty() {
        println!("No valid menu items found. Please check your configs/menu.txt format.");
        return;
    }
    
    // Group commands by path
    let grouped_commands = group_menu_commands(&commands);
    println!("Created {} menu groups", grouped_commands.len());
    
    let main_window = MainWindow::new().unwrap();
    
    // Convert GroupedMenuEntry structs to slint MenuEntry objects
    let menu_items: Vec<MenuEntry> = grouped_commands
        .iter()
        .map(|group| {
            // Convert actions to Vec<SharedString> first
            let actions: Vec<SharedString> = group.actions
                .iter()
                .map(|a| a.clone().into())
                .collect();
            
            // Convert commands to Vec<SharedString> first
            let commands: Vec<SharedString> = group.commands
                .iter()
                .map(|c| c.clone().into())
                .collect();
            
            // Create the MenuEntry with the converted vectors
            // Use the new conversion pattern from the changelog
            MenuEntry {
                program: group.program.clone().into(),
                path_name: group.path_name.clone().into(),
                actions: Rc::new(VecModel::from(actions)).into(),
                commands: Rc::new(VecModel::from(commands)).into(),
            }
        })
        .collect();
    
    // Convert to Slint's model
    let model = Rc::new(VecModel::from(menu_items));
    main_window.set_menu_items(ModelRc::from(model));
    
    let window_weak = main_window.as_weak();
    
    main_window.on_run_command(move |command_text| {
        run_command_async(command_text.to_string(), window_weak.clone());
    });
    
    println!("Starting UI...");
    main_window.run().unwrap();
}