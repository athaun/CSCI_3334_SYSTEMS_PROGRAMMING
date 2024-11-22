mod json;
mod assets;

use assets::*;
use core::time::Duration;
use std::thread::sleep;

fn main() {
    // Store the assets as using box (heap) pointers to allow iteration
    let mut assets_to_fetch: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin::new(
            "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd",
            "bitcoin_prices.txt",
        )),
        Box::new(Ethereum::new(
            "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd",
            "ethereum_prices.txt",
        )),
        Box::new(SP500::new(
            "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC",
            "sp500_prices.txt",
        )),
    ];
    
    // Loop forever, fetching then saving the asset prices to a file.
    loop {
        for asset in &mut assets_to_fetch {
            let price = asset.fetch_price();
            asset.save_to_file();
            println!("Price fetched and saved: ${:.2}", price);
        }

        // Wait 10 seconds
        sleep(Duration::from_secs(10));
    }
}
