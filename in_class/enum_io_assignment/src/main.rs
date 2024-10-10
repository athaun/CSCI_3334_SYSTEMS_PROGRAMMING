use std::fs;
use std::io::{self, Write};
use std::path::Path;

enum FileOperation {
    Create(String),
    Rename(String, String),
    Write(String, String)
}

impl FileOperation {
    fn get_input(prompt: String) -> String {
        println!("{}", prompt);
        let mut input : String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        return input.trim().to_string()
    }

    fn validate_file(filename: &String) -> bool {
        return Path::new(&filename).exists();
    }
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::Create(filename) => {
            if FileOperation::validate_file(&filename) {
                println!("File already exists!");
                return;
            }
            fs::File::create(&filename).unwrap();
            println!("File '{}' created successfully.", filename);
        }
        FileOperation::Rename(old_name, new_name) => {
            if !FileOperation::validate_file(&old_name) {
                println!("File doesn't exist!");
                return;
            }
            fs::rename(&old_name, &new_name).unwrap();
            println!("File renamed from '{}' to '{}' successfully.", old_name, new_name);
        },
        FileOperation::Write(filename, contents) => {
            if (!FileOperation::validate_file(&filename)) {
                FileOperation::Create(filename);
            }

            // TODO: finish this
        }
    }
}

fn main() {
    for _ in 0..2 {
        let mut input = FileOperation::get_input("Choose an operation:\n1. Create a new file\n2. Rename an existing file\n3. Write to a file".to_string());

        match input.trim() {
            "1" => {
                input = FileOperation::get_input("Input new file name:".to_string());
                perform_operation(FileOperation::Create(input));
            }
            "2" => {
                let old_file = FileOperation::get_input("Input current file name:".to_string());
                let new_file = FileOperation::get_input("Input new file name:".to_string());
                perform_operation(FileOperation::Rename(old_file, new_file))
            }, 
            "3" => {
                let file_name = FileOperation::get_input("Input file name:".to_string());
                let contents = FileOperation::get_input("Input file contents".to_string());
                perform_operation(FileOperation::Write(file_name, contents));
            }
            _ => println!("Invalid choice"),
        }
    }
}