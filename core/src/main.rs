use std::env;
use tokio::process::Command;
use tokio::time::{sleep, Duration};
use tokio::fs as tokio_fs;
use std::io;

// async fn run_command(command: &str, args: &[&str]) -> io::Result<Vec<u8>> {
//     let output = Command::new(command)
//         .args(args)
//         .output()
//         .await?;
//     Ok(output.stdout)
// }

async fn app_start(app: &str, profile: &str, profile_part: &str) {
    let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let unit_name = format!("app.{}-{}", app, profile_part);
    let slice_name = format!("user.{}.app.{}.slice", user, app);
    let args = [
        "--user",
        "--scope",
        "--slice",
        &slice_name,
        "--unit",
        &unit_name,
        app,
        "--profile",
        profile,
    ];

    match Command::new("systemd-run").args(&args).output().await {
        Ok(output) => {
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => eprintln!("Failed to execute app_start: {}", e),
    }
}

async fn app_status(app: &str, profile_part: &str) -> Option<String> {
    let scope = format!("app.{}-{}.scope", app, profile_part);
    let args = ["--user", "status", &scope];
    match Command::new("systemctl").args(&args).output().await {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            println!("{}", stdout);
            Some(stdout)
        }
        Err(e) => {
            eprintln!("Failed to get app status: {}", e);
            None
        }
    }
}

async fn app_full_syspath(app: &str, profile_part: &str) -> Option<String> {
    if let Some(status) = app_status(app, profile_part).await {
        for line in status.lines() {
            if line.contains("CGroup:") {
                if let Some(idx) = line.find(':') {
                    let cgroup_info = line[(idx + 1)..].trim();
                    if let Some(token) = cgroup_info.split_whitespace().next() {
                        let full_path = format!("/sys/fs/cgroup/{}", token);
                        println!("{}", full_path);
                        return Some(full_path);
                    }
                }
            }
        }
    }
    None
}

async fn app_freeze(app: &str, profile_part: &str) {
    if let Some(syspath) = app_full_syspath(app, profile_part).await {
        let freeze_path = format!("{}/cgroup.freeze", syspath);
        if let Err(e) = tokio_fs::write(&freeze_path, "1").await {
            eprintln!("Failed to freeze app: {}", e);
        } else {
            println!("App frozen.");
        }
    }
}

async fn app_unfreeze(app: &str, profile_part: &str) {
    if let Some(syspath) = app_full_syspath(app, profile_part).await {
        let freeze_path = format!("{}/cgroup.freeze", syspath);
        if let Err(e) = tokio_fs::write(&freeze_path, "0").await {
            eprintln!("Failed to unfreeze app: {}", e);
        } else {
            println!("App unfrozen.");
        }
    }
}

async fn app_kill(app: &str, profile_part: &str) {
    if let Some(syspath) = app_full_syspath(app, profile_part).await {
        let kill_path = format!("{}/cgroup.kill", syspath);
        if let Err(e) = tokio_fs::write(&kill_path, "1").await {
            eprintln!("Failed to kill app: {}", e);
        } else {
            println!("App killed.");
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <command> <app> <profile>", args[0]);
        return;
    }

    let command = &args[1];
    let app = args[2].clone();
    let mut profile = args[3].clone();

    // Remove trailing slash if exists
    if profile.ends_with('/') {
        profile.pop();
    }
    // Get the last part from the profile path (profile_part)
    let profile_part = profile.split('/').last().unwrap_or("").to_string();

    println!("profile-%&eol: {}", profile);
    println!("Profile part: {}", profile_part);
    println!("Profile: {}", profile);

    match command.as_str() {
        "start" => {
            // Clone to move into the async spawn, ensuring a 'static lifetime.
            let app_spawn = app.clone();
            let profile_spawn = profile.clone();
            let profile_part_spawn = profile_part.clone();
            tokio::spawn(async move {
                app_start(&app_spawn, &profile_spawn, &profile_part_spawn).await;
            });
            // Wait for some time to allow the unit to be created
            sleep(Duration::from_secs(2)).await;
            app_full_syspath(&app, &profile_part).await;
        }
        "status" => {
            app_status(&app, &profile_part).await;
        }
        "syspath" => {
            app_full_syspath(&app, &profile_part).await;
        }
        "freeze" => {
            app_freeze(&app, &profile_part).await;
        }
        "unfreeze" => {
            app_unfreeze(&app, &profile_part).await;
        }
        "kill" => {
            app_kill(&app, &profile_part).await;
        }
        _ => {
            eprintln!("Unknown function: {}", command);
        }
    }
}
