use ureq;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Price {
    usd: f32,
    btc: f32,
    eth: f32
}

#[derive(Debug, Deserialize)]
struct Crypto {
    bitcoin: Price,
    ethereum: Price
}

fn main() {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum,bitcoin&vs_currencies=usd,btc,eth";

    let req = ureq::get(url).call().unwrap();
    let content = req.into_json::<Crypto>().unwrap();

    println!("{:?}", content);
}
