use std::fs;
use std::io;

fn main() -> io::Result<()> {
    println!("Please run a specific example with cargo run --bin <example>");
    println!("For example: cargo run --bin hello");
    println!("Choices are the following");

    // List all files in the src/bin directory
    let entries = fs::read_dir("src/bin")?;

    println!("Available files:");
    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    if let Some(filename) = path.file_name() {
                        if let Some(filename_str) = filename.to_str() {
                            // Exclude the main.rs file
                            if filename_str != "main.rs" {
                                println!("{}", filename_str);
                            }
                        }
                    }
                }
            }
            Err(_) => {
                // Handle the error, e.g., by logging, panicking, or simply ignoring it.
            }
        }
    }

    Ok(())
}
