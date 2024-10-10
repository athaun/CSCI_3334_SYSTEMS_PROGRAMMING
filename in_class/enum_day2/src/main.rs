enum Pets {
    Dog(String, u8),
    Hamster(String)
}

impl Pets {
    fn introduce_yourself(&self) {
        match &self {
            Pets::Dog(name, age) => println!("Hi, my name is {}, and I am {} years old", name, age),
            Pets::Hamster(name) => println!("Hi my name is {}", name)
        }
    }
}

fn main() {
    let a = Pets::Dog("Bob".to_string(), 5);
    a.introduce_yourself();

    let b = Pets::Hamster("Jerry".to_string());
    b.introduce_yourself();

}
