use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug)]
struct Car {
    make: String,
    model: String,
    year: u16,
}

fn main() {
    let mut buffer = String::new();

    println!("Enter the make of your car: ");
    io::stdin().read_line(&mut buffer).unwrap();
    let make = buffer.trim().to_string();
    
    println!("Enter the model of your car: ");
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    
    println!("Enter the year of your car: ");
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let year: u16 = buffer.trim().parse().unwrap();

    let car = Car { make, model, year };

    let serialized_car = format!("{:?}", car);

    let path = "user_info.txt";
    let mut file = File::create(&path).expect("Could not create file");
    file.write_all(serialized_car.as_bytes()).expect("Could not write to file");

    let mut file = File::open(&path).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file");
    println!("File contents:\n{}", contents);
}