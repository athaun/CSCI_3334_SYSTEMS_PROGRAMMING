use std::process::Command;

fn main() {
    let new_file = "hello.txt";
    let command = format!("touch {}", new_file);

    println!("{}", command);

    let output = Command::new("sh").arg("-c").arg(command).spawn();
}