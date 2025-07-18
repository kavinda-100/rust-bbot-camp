use std::fs;

pub fn start() {
    // Specify the path to the directory you want to create
    let path = "./src/file_system/data";
    // Call the function to create the directory
    create_directory(path);

    // Specify the path to the file you want to create
    let file_path = "./src/file_system/data/temp_file.txt";
    // Specify the content you want to write to the file
    let file_content = "This is a temporary file created by Rust.";
    // Call the function to create the file
    create_file(file_path, file_content);

    // read the file
    read_file(file_path);

    // remove the file
    let r_file_path = "./src/file_system/data/remove_file.txt";
    // create a file to remove
    create_file(r_file_path, "This file will be removed.");
    // wait for a moment to ensure the file is created
    std::thread::sleep(std::time::Duration::from_secs(2));
    remove_file(r_file_path);
}

// create a directory
fn create_directory(path: &str) {
    // check if the directory already exists
    if fs::metadata(path).is_ok() {
        println!("Directory already exists.");
        return;
    }
    let res = fs::create_dir(path);
    match res {
        Ok(_) => println!("Directory created successfully."),
        Err(e) => println!("Failed to create directory: {}", e),
    }
}

// create a file in the directory
fn create_file(path: &str, file_content: &str) {
    // create the file
    let res = fs::write(path, file_content);
    match res {
        Ok(_) => println!("File created successfully at {}", path),
        Err(e) => println!("Failed to create file: {}", e),
    }
}

// read a file
fn read_file(path: &str) {
    // check if the file exists
    if fs::metadata(path).is_err() {
        println!("File does not exist.");
        return;
    }
    // read the file content
    // let res = fs::read(path); // returns a Result<Vec<u8>, std::io::Error>
    let res = fs::read_to_string(path);
    match res {
        Ok(content) => println!("File content:= {}", content),
        Err(e) => println!("Failed to read file: {}", e),
    }
}

// remove a file
fn remove_file(path: &str) {
    // check if the file exists
    if fs::metadata(path).is_err() {
        println!("File does not exist.");
        return;
    }
    let res = fs::remove_file(path);
    match res {
        Ok(_) => println!("File removed successfully."),
        Err(e) => println!("Failed to remove file: {}", e),
    }
}
