use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

fn test() {

    let mut exe_dir = env::current_exe().expect("failed to get current executable path");
    exe_dir.pop();
    
    let output = Command::new("cat")
        .arg("example.txt")
        .output()
        .expect("failed to execute process");

    println!("status: {}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    test();
}