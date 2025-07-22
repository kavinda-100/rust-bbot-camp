use std::process::Command;

pub fn start() {
    let res = Command::new("which").args(&["python3"]).output();
    match res {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("Command executed successfully:\n{}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Command failed:\n{}", stderr);
            }
        }
        Err(e) => eprintln!("Failed to execute command: {}", e),
    }

    // create a folder named "my_folder"
    let folder_name = "my_folder";
    let res2 = Command::new("mkdir")
        .args(&["-p", folder_name]) // -p option to avoid error if the folder already exists
        .spawn();
    println!("doing something else while creating a folder...");
    match res2 {
        Ok(mut child) => match child.wait() {
            Ok(status) => {
                if status.success() {
                    println!("Folder '{}' created successfully.", folder_name);
                } else {
                    eprintln!("Failed to create folder '{}'.", folder_name);
                }
            }
            Err(e) => eprintln!("Failed to wait for command: {}", e),
        },
        Err(e) => eprintln!("Failed to execute command to create folder: {}", e),
    }
}
