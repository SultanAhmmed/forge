use std::{fs, path::Path};

fn main() {
    let args: Vec<String> = std::env::args().collect(); // Get the folder path from the command line arguments

    if args.len() < 2 {
        println!("Usage: forge organize <folder_path>");
        return;
    }

    let folder_path = &args[1];
    let path = Path::new(folder_path);

    if !path.exists() {
        println!("Folder does not exitst: {}", folder_path);
        return;
    }

    println!("Scanning folder: {}", folder_path);

    // Read directory contents
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let file_name = entry.file_name();
                        println!("Found: {:?}", file_name);
                    }
                    Err(e) => println!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => println!("Error reading directory: {}", e),
    }
}
