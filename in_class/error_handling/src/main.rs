use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

fn read_username_from_file_3ver() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let username = read_username_from_file().unwrap();
}