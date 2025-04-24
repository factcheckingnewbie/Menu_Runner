use std::env;
use std::path::Path;
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <start|status|freeze|unfreeze|kill> <app> <profile>", args[0]);
        std::process::exit(1);
    }

    let func = &args[1];
    let app = &args[2];
    let profile = args[3].trim_end_matches('/').to_string();

    let profile_part = Path::new(&profile).file_name().unwrap().to_str().unwrap();
    let container_name = format!("app.{}-{}", app, profile_part);
    let container_profile_path = "/home/firefoxuser/profile";
    let display = env::var("DISPLAY").unwrap();

    match func.as_str() {
        "start" => app_start(&container_name, &profile, container_profile_path, &display).await,
        "status" => app_status(&container_name).await,
        "freeze" => app_freeze(&container_name).await,
        "unfreeze" => app_unfreeze(&container_name).await,
        "kill" => app_kill(&container_name).await,
        _ => show_error(&format!("Unknown function: {}", func)),
    }
}
//

async fn app_start(container_name: &str, profile: &str, container_profile_path: &str, display: &str) {
    let user_id = users::get_current_uid();
    let status = Command::new("docker")
        .args([
            "run",
            "-d",
            "--rm",
            "--name",
            container_name,
            "-e",
            &format!("DISPLAY={}", display),
            "-v",
            "/tmp/.X11-unix:/tmp/.X11-unix:z",
            "-v",
            &format!("/run/user/{}/pulse:/run/user/{}/pulse", user_id, user_id),
            "-v",
            "/etc/machine-id:/etc/machine-id",
            "-e",
            &format!("PULSE_SERVER=unix:/run/user/{}/pulse/native", user_id),
            "-v",
            &format!("{}:{}", profile, container_profile_path),
            "--user",
            &format!("{}:{}", user_id, users::get_current_gid()),
            "fedora-x11-test",
            "firefox",
            "-profile",
            container_profile_path,
        ])
        .status()
        .await
        .expect("Failed to execute Docker command");

    if !status.success() {
        show_error("Failed to start container");
    }
}

//

//
// async fn app_start(container_name: &str, profile: &str, container_profile_path: &str, display: &str) {
//     let user_id = users::get_current_uid();
//     let status = Command::new("docker")
//         .args([
//             "run",
//             "-d",
//             "--rm",
//             "--name",
//             container_name,
//             "-e",
//             &format!("DISPLAY={}", display),
//             "-v",
//             "/tmp/.X11-unix:/tmp/.X11-unix:z",
//             "-v",
//             &format!("/run/user/{}/pulse:/run/user/{}/pulse", user_id, user_id),
//             "-v",
//             "/etc/machine-id:/etc/machine-id",
//             "-e",
//             &format!("PULSE_SERVER=unix:/run/user/{}/pulse/native", user_id),
//             "-v",
//             &format!("{}:{}", profile, container_profile_path),
//             "--user",
//             &format!("{}:{}", user_id, users::get_current_gid()),
//             "fedora-x11-test",
//             "firefox",
//             "-profile",
//             container_profile_path,
//         ])
//         .status()
//         .await
//         .expect("Failed to execute Docker command");
// 
//     if !status.success() {
//         show_error("Failed to start container");
//     }
// }
// //
// async fn app_start(container_name: &str, profile: &str, container_profile_path: &str, display: &str) {
//     let status = Command::new("docker")
//         .args([
//             "run",
//             "-d",
//             "--rm",
//             "--name",
//             container_name,
//             "-e",
//             &format!("DISPLAY={}", display),
//             "-v",
//             "/tmp/.X11-unix:/tmp/.X11-unix:z",
//             "-v",
//             &format!("{}:{}", profile, container_profile_path),
//             "--user",
//             &format!("{}:{}", users::get_current_uid(), users::get_current_gid()),
//             "fedora-x11-test",
//             "firefox",
//             "-profile",
//             container_profile_path,
//         ])
//         .status()
//         .await
//         .expect("Failed to execute Docker command");
// 
//     if !status.success() {
//         show_error("Failed to start container");
//     }
// }

async fn app_status(container_name: &str) {
    let output = Command::new("docker")
        .args(["ps", "-a", "--filter", &format!("name={}", container_name)])
        .output()
        .await
        .expect("Failed to execute Docker command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

async fn app_freeze(container_name: &str) {
    let status = Command::new("docker")
        .args(["pause", container_name])
        .status()
        .await
        .expect("Failed to execute Docker command");

    if !status.success() {
        show_error("Failed to freeze container");
    }
}

async fn app_unfreeze(container_name: &str) {
    let status = Command::new("docker")
        .args(["unpause", container_name])
        .status()
        .await
        .expect("Failed to execute Docker command");

    if !status.success() {
        show_error("Failed to unfreeze container");
    }
}

async fn app_kill(container_name: &str) {
    let status = Command::new("docker")
        .args(["kill", container_name])
        .status()
        .await
        .expect("Failed to execute Docker command");

    if !status.success() {
        show_error("Failed to kill container");
    }
}

fn show_error(message: &str) {
    eprintln!("{}", message);
    std::process::exit(1);
}


