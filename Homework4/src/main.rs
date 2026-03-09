use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(directory_path) => {
            Command::new("ls")
                .arg(directory_path)
                .status()
                .expect("Failed to execute ls");
        }
        FileOperation::Display(file_path) => {
            Command::new("cat")
                .arg(file_path)
                .status()
                .expect("Failed to execute cat");
        }
        FileOperation::Create(file_path, content) => {
            let command = format!("echo '{}' > {}", content, file_path);
            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("Failed to create file");
            if output.status.success() {
                println!("File '{}' created successfully.", file_path);
            } else {
                eprintln!("Failed to create file '{}'.", file_path);
            }
        }
        FileOperation::Remove(file_path) => {
            let status = Command::new("rm")
                .arg(&file_path)
                .status()
                .expect("Failed to remove file");
            if status.success() {
                println!("File '{}' removed successfully.", file_path);
            } else {
                eprintln!("Failed to remove file '{}'.", file_path);
            }
        }
        FileOperation::Pwd => {
            Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");
        }
    }
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");
        println!();

        let choice = read_line("Enter your choice (0-5): ");

        let operation = match choice.as_str() {
            "1" => {
                let path = read_line("Enter directory path: ");
                FileOperation::List(path)
            }
            "2" => {
                let path = read_line("Enter file path: ");
                FileOperation::Display(path)
            }
            "3" => {
                let path = read_line("Enter file path: ");
                let content = read_line("Enter content: ");
                FileOperation::Create(path, content)
            }
            "4" => {
                let path = read_line("Enter file path: ");
                FileOperation::Remove(path)
            }
            "5" => FileOperation::Pwd,
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Please enter a number between 0 and 5.");
                continue;
            }
        };

        perform_operation(operation);
    }
}
