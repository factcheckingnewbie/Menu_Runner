use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::rc::Rc;
use std::path::Path;
use std::thread;
use slint::{ModelRc, VecModel, Weak};

// Define the UI inline with proper imports
slint::slint! {
    import { VerticalBox, ListView } from "std-widgets.slint";

    struct MenuEntry {
        number: string,
        name: string,
    }

    export component MainWindow inherits Window {
        title: "Menu Runner";
        width: 400px;
        height: 500px;
        
        callback run_command(int);
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
                text: "No menu items found in menu.txt";
                visible: menu_items.length <= 0;
                color: red;
            }
            
            ListView {
                visible: menu_items.length > 0;
                for item[i] in menu_items: Rectangle {
                    height: 40px;
                    
                    VerticalLayout {
                        padding: 5px;
                        
                        Text {
                            text: item.number + ". " + item.name;
                            font-size: 16px;
                        }
                    }
                    
                    TouchArea {
                        width: 100%;
                        height: 100%;
                        clicked => {
                            run_command(i);
                        }
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

struct MenuCommand {
    name: String,
    command: String,
}

fn load_menu() -> Vec<MenuCommand> {
    // Check if menu.txt exists
    if !Path::new("menu.txt").exists() {
        println!("ERROR: menu.txt file not found!");
        return Vec::new();
    }
    
    let file = match File::open("menu.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open menu.txt: {}", e);
            return Vec::new();
        }
    };
    
    let reader = BufReader::new(file);
    let mut commands = Vec::new();
    let mut line_number = 0;
    
    for line in reader.lines() {
        line_number += 1;
        match line {
            Ok(line) => {
                if line.trim().is_empty() {
                    println!("Line {}: Empty line, skipping", line_number);
                    continue;
                }
                
                // Parse quoted strings
                let mut parts: Vec<String> = Vec::new();
                let mut current_part = String::new();
                let mut in_quotes = false;
                
                for c in line.chars() {
                    if c == '"' {
                        // Toggle quote state
                        in_quotes = !in_quotes;
                        
                        // If we're ending a quoted section, save it
                        if !in_quotes && !current_part.is_empty() {
                            parts.push(current_part.clone());
                            current_part.clear();
                        }
                    } else if in_quotes {
                        // Inside quotes, accumulate characters
                        current_part.push(c);
                    }
                    // Skip spaces outside quotes
                }
                
                // Clean up trailing quote in case it exists
                if in_quotes && !current_part.is_empty() {
                    current_part = current_part.trim_end_matches('"').to_string();
                    parts.push(current_part);
                }
                
                if parts.len() >= 2 {
                    println!("Line {}: Added menu item \"{}\" with command \"{}\"", 
                             line_number, parts[0], parts[1]);
                    commands.push(MenuCommand {
                        name: parts[0].clone(),
                        command: parts[1].clone(),
                    });
                } else {
                    println!("Line {}: Invalid format (expected two quoted sections): {}", 
                             line_number, line);
                }
            },
            Err(e) => println!("Error reading line {}: {}", line_number, e),
        }
    }
    
    println!("Loaded {} menu items from menu.txt", commands.len());
    commands
}

fn run_command_async(command_text: String, command_name: String, window: Weak<MainWindow>) {
    thread::spawn(move || {
        // Clone command_name since we'll use it multiple times
        let display_name = command_name.clone();
        
        // Set status to indicate we're starting the command
        window.upgrade_in_event_loop(move |handle| {
            handle.set_status_text(format!("Running: {}", display_name).into());
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
                
                println!("Command '{}' completed with status: {}", command_name, status);
                println!("Output: {}", stdout);
                
                if !stderr.is_empty() {
                    println!("Errors: {}", stderr);
                }
                
                // Update UI with status
                let status_message = if status.success() {
                    format!("Command '{}' completed successfully", command_name)
                } else {
                    format!("Command '{}' failed with status {}", command_name, status)
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
                let error_message = format!("Failed to execute command '{}': {}", command_name, e);
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
        println!("No valid menu items found. Please check your menu.txt format.");
        return;
    }
    
    let commands_rc = Rc::new(commands);
    
    let main_window = MainWindow::new().unwrap();
    
    // Create a vector of MenuEntry objects for Slint
    let menu_items: Vec<MenuEntry> = commands_rc
        .iter()
        .enumerate()
        .map(|(i, cmd)| {
            println!("Adding menu item {}: {}", i+1, cmd.name);
            MenuEntry {
                number: (i + 1).to_string().into(),
                name: cmd.name.clone().into(),
            }
        })
        .collect();
    
    println!("Created {} menu entries for UI", menu_items.len());
    
    // Convert to Slint's model
    let model = Rc::new(VecModel::from(menu_items));
    main_window.set_menu_items(ModelRc::from(model));
    
    let commands_for_callback = commands_rc.clone();
    let window_weak = main_window.as_weak();
    
    main_window.on_run_command(move |index| {
        let command = &commands_for_callback[index as usize];
        let command_text = command.command.clone();
        let command_name = command.name.clone();
        
        run_command_async(command_text, command_name, window_weak.clone());
    });
    
    println!("Starting UI...");
    main_window.run().unwrap();
}
