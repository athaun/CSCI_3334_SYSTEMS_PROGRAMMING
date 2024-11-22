/*
    This module contains representations of the JSON responses recieved from the various APIs
*/

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BTCPrice {
    pub bitcoin: Price
}

#[derive(Debug, Deserialize)]
pub struct ETHPrice {
    pub ethereum: Price
}

// Only substruct required for coingecko requests
#[derive(Debug, Deserialize)]
pub struct Price {
    pub usd: f32,
}

// SP500 and its many substructs
#[derive(Debug, Deserialize)]
pub struct SP500Price {
    pub chart: Chart,
}

#[derive(Debug, Deserialize)]
pub struct Chart {
    pub result: Vec<Result>,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    pub meta: Meta,
    pub indicators: Indicators,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    #[serde(rename = "regularMarketPrice")]
    pub regular_market_price: f32,
}

#[derive(Debug, Deserialize)]
pub struct Indicators {
    pub quote: Vec<Quote>,
}

#[derive(Debug, Deserialize)]
pub struct Quote {
    pub close: Vec<Option<f32>>,
}