// filename: regex_test.rs
use regex::Regex;
use std::env;

fn main() {
    // Sample action strings to test
    let test_strings = vec![
        "freeze:unfreeze",
        "start:stop",
        "play:pause:blue",
        "single_action",
        "compile:run:green"
    ];
    
    // Get command line argument or use default toggle state
    let args: Vec<String> = env::args().collect();
    let toggle_state = args.get(1).map_or(false, |arg| arg == "true");
    
    println!("Toggle state: {}\n", toggle_state);
    
    // Process each test string
    for action in &test_strings {
        println!("Original action: \"{}\"", action);
        
        // Extract parts using regex
        let re = Regex::new(r"^([^:]+)(?::([^:]+))?(?::([^:]+))?$").unwrap();
        let caps = re.captures(action);
        
        if let Some(captures) = caps {
            // First part (always present) - shown when not toggled
            let first_part = captures.get(1).map_or("", |m| m.as_str());
            
            // Second part (optional) - shown when toggled
            let second_part = captures.get(2).map_or("", |m| m.as_str());
            
            // Third part (optional) - color information
            let color = captures.get(3).map_or("default", |m| m.as_str());
            
            // Display current text based on toggle state
            let display_text = if toggle_state {
                if second_part.is_empty() { first_part } else { second_part }
            } else {
                first_part
            };
            
            // Display the command that would be executed
            let command_text = if toggle_state {
                if second_part.is_empty() { first_part } else { second_part }
            } else {
                first_part
            };
            
            println!("  Button text: \"{}\"", display_text);
            println!("  Command to run: \"{}\"", command_text);
            println!("  Button color: {}", color);
        } else {
            println!("  No pattern match, using original: \"{}\"", action);
        }
        
        println!();
    }
    
    // Show how this would work in Slint code
    println!("Equivalent Slint expression for button text:");
    println!("  is-toggled ? ");
    println!("    (action.match(\"^([^:]+):([^:]+)\") ? action.match(\"^([^:]+):([^:]+)\")[2] : action)");
    println!("    : (action.match(\"^([^:]+):([^:]+)\") ? action.match(\"^([^:]+):([^:]+)\")[1] : action)");
}
