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
    let folder_name = "src/external_process/my_folder";
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

    // create a file named "my_file.txt" in the "my_folder"
    let file_name = "src/external_process/my_folder/my_file.txt";
    let res3 = Command::new("touch").arg(file_name).spawn();
    println!("doing something else while creating a file...");
    match res3 {
        Ok(mut child) => match child.wait() {
            Ok(status) => {
                if status.success() {
                    println!("File '{}' created successfully.", file_name);
                } else {
                    eprintln!("Failed to create file '{}'.", file_name);
                }
            }
            Err(e) => eprintln!("Failed to wait for command: {}", e),
        },
        Err(e) => eprintln!("Failed to execute command to create file: {}", e),
    }

    // add content to the file
    let content = "Hello, this is a test file!";
    let file_name = "src/external_process/my_folder/my_file.txt";
    let res4 = Command::new("sh") // Use "sh" to run the command in a shell
        .arg("-c") // -c option to run the command in a shell
        .arg(format!("echo '{}' > {}", content, file_name))
        .spawn();
    println!("doing something else while adding content to the file...");
    match res4 {
        Ok(mut child) => match child.wait() {
            Ok(status) => {
                if status.success() {
                    println!("Content added to file '{}' successfully.", file_name);
                } else {
                    eprintln!("Failed to add content to file '{}'.", file_name);
                }
            }
            Err(e) => eprintln!("Failed to wait for command: {}", e),
        },
        Err(e) => eprintln!("Failed to execute command to add content to file: {}", e),
    }
}
