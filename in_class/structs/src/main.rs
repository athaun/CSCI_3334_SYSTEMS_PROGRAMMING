use std::mem;

#[derive(Debug)]
struct Car {
    color: String,
    make: String,
    year: u16
}

impl Car {
    fn new(color: String, make: String, year: u16) -> Car {
        Car {
            color: color,
            make: make,
            year: year
        }
    }

    fn honk_honk(&self) {
        println!("Car with color {} HONK HONK!", self.color);
    }

    fn upgrade(&mut self, year: u16) {
        self.year = year;
    }
}

#[derive(Debug)]
struct ComputingDevice {
    manufacturer: String,
    cpu: String,
    RAM: u16
}

impl ComputingDevice {
    fn new(manufacturer: String, cpu: String, RAM: u16) -> Self {
        ComputingDevice {
            manufacturer: manufacturer,
            cpu: cpu,
            RAM: RAM
        }
    }
    
    fn upgrade(&mut self, RAM: u16) {
        self.RAM = RAM;
    }

    fn self_destruct(self) {
        println!("YOUR COMPUTER IS NO MORE!\n{:?}", self);
    }
}

fn main() {
    println!("Hello, world!");

    let mut mc = Car::new("white".to_string(), "toyota".to_string(), 2015);
    println!("{:?}", mc);
    mc.honk_honk();
    mc.honk_honk();
    mc.upgrade(2017);
    println!("{:?}", mc);

    println!("Size of Car: {} bytes", mem::size_of::<Car>());
    println!("Alignment of Car: {} bytes", mem::align_of::<Car>());

    let mut cd = ComputingDevice::new("Apple".to_string(), "M1 Pro".to_string(), 16);
    println!("\n\n{:?}", cd);
    cd.upgrade(32);
    println!("{:?}", cd);
    cd.self_destruct();

    // cd.upgrade(128); // Won't compile, I took ownership in the self destruct scope and dropped it at the end of scope.


}
