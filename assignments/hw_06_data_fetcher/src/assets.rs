/*
    This crate contains the asset structs and price fetching/saving methods for each.
    It also contains fetching and saving helpers.
*/

use crate::json;

use std::fs::OpenOptions;
use std::io::Write;
use ureq;

// Asset structs
#[derive(Debug)]
pub struct Bitcoin {
    api_address: String,
    file_name: String,
    cached_price: Option<f32>
}

#[derive(Debug)]
pub struct Ethereum {
    api_address: String,
    file_name: String,
    cached_price: Option<f32>
}

#[derive(Debug)]
pub struct SP500 {
    api_address: String,
    file_name: String,
    cached_price: Option<f32>
}

// Asset method implementations
pub trait Pricing {
    fn fetch_price(&mut self) -> f32;
    fn save_to_file(&self) -> ();

    fn new(url: &str, file: &str) -> Self
    where Self: Sized;
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> f32 {
        // Call the fetch_price helper, passing a closure to save the BTCPrice struct and cache it
        return fetch_price(self.api_address.as_str(), |parsed: json::BTCPrice| {
            self.cached_price = Some(parsed.bitcoin.usd as f32);
            return self.cached_price;
        });
    }

    fn save_to_file(&self) {
        // Exit early if there is no price Cached
        if let None = self.cached_price { return; }
        // Save the price into the file or print if it fails
        if let Err(e) = save_price_to_file(&self.file_name, "BTC", self.cached_price.unwrap()) {
            eprintln!("Failed to save BTC price: {}", e);
        }
    }

    fn new(api_address: &str, file_name: &str) -> Bitcoin {
        Self {
            api_address: api_address.to_string(),
            file_name: file_name.to_string(),
            cached_price: None,
        }
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> f32 {
        return fetch_price(self.api_address.as_str(), |parsed: json::ETHPrice| {
            self.cached_price = Some(parsed.ethereum.usd as f32);
            return self.cached_price;
        });
    }

    fn save_to_file(&self) {
        if let None = self.cached_price { return; }
        if let Err(e) = save_price_to_file(&self.file_name, "Eth", self.cached_price.unwrap()) {
            eprintln!("Failed to save Eth price: {}", e);
        }
    }

    fn new(api_address: &str, file_name: &str) -> Ethereum {
        Self {
            api_address: api_address.to_string(),
            file_name: file_name.to_string(),
            cached_price: None,
        }
    }
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) -> f32 {
        return fetch_price(self.api_address.as_str(), |parsed: json::SP500Price| {
            if !parsed.chart.result.is_empty() {
                self.cached_price = Some(parsed.chart.result[0].meta.regular_market_price);
                return self.cached_price;
            } else {
                return None;
            }
        });
    }

    fn save_to_file(&self) {
        if let None = self.cached_price { return; }
        if let Err(e) = save_price_to_file(&self.file_name, "SP500", self.cached_price.unwrap()) {
            eprintln!("Failed to save SP500 price: {}", e);
        }
    }

    fn new(api_address: &str, file_name: &str) -> SP500 {
        Self {
            api_address: api_address.to_string(),
            file_name: file_name.to_string(),
            cached_price: None,
        }
    }
}

// Helper methods for saving and fetching data

fn save_price_to_file(file_name: &str, asset_name: &str, price: f32) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;
        
    writeln!(file, "{} Price: ${:.2}", asset_name, price)?;
    println!("Saved {} price to {}", asset_name, file_name);
    Ok(())
}

// Fetches the price from the provided API URL
// The function provided (`parse_price`) is used to extract the price from the response.
fn fetch_price<Price, F>(
    api_url: &str,
    mut parse_price: F,
) -> f32
where
    Price: for<'de> serde::Deserialize<'de>, // Enforce the serde deserialize trait in the Price template
    F: FnMut(Price) -> Option<f32>, // FnMut because this closure modifies self.cached_price
{
    match ureq::get(api_url).call() {
        Ok(response) => {
            // The API responded, cast the data to the appropriate Price json struct
            match response.into_json::<Price>() {
                Ok(parsed) => {
                    // Execute the parsing closure and return the value it finds
                    if let Some(price) = parse_price(parsed) {
                        return price;
                    } else {
                        eprintln!("Failed to extract price from response");
                        return -1.0;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse response: {}", e);
                    return -1.0;
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch price: {}", e);
            return -1.0;
        }
    }
}