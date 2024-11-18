use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Price {
    usd: i32,
}

pub trait Pricing {
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self) -> ();
}

#[derive(Debug, Deserialize)]
struct BTCPrice {
    bitcoin: Price
}

#[derive(Debug)]
struct Bitcoin {
    api_address: String,
    file_name: String,
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> f32 {
        return 0.0;
    }

    fn save_to_file(&self) {
        println!("Saved to {}", self.file_name);
    }
}

#[derive(Debug, Deserialize)]
struct ETHPrice {
    etherium: Price
}

struct Etherium {
    api_address: String,
    file_name: String,
}

impl Pricing for Etherium {
    fn fetch_price(&self) -> f32 {
        return 0.0;
    }

    fn save_to_file(&self) {
        println!("Saved to {}", self.file_name);
    }
}

#[derive(Debug, Deserialize)]
struct SP00Price {
    // idk yet
}

struct SP500 {
    api_address: String,
    file_name: String,
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> f32 {
        return 0.0;
    }

    fn save_to_file(&self) {
        println!("Saved to {}", self.file_name);
    }
}

fn main() {
    // Only use serde and other lib in dog api example
    // no async, no smart pointers
    let b = Bitcoin{api_address: "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string(), file_name: "test".to_string()};
    println!("{:?}", b);
}
