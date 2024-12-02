# Asset Price Fetcher

This program fetches real-time prices for various assets such as Bitcoin (BTC), Ethereum (ETH), and the S&P 500 Index, and saves them to a text file. It retrieves the asset prices from public APIs and saves the results to local files for future reference. The program fetches prices every 10 seconds and saves them.

### Supported Assets
- **Bitcoin (BTC)**: Price in USD, provided by the CoinGecko API.
- **Ethereum (ETH)**: Price in USD, provided by the CoinGecko API.
- **S&P 500 Index (SP500)**: The regular market price for the S&P 500, provided by the Yahoo Finance API.

## Modules Overview

### 1. `json.rs`
This module defines the structure of the JSON responses received from the APIs. Each asset has its own structure:
- **BTCPrice**: Represents the response for Bitcoin with a `bitcoin` field containing a `Price`.
- **ETHPrice**: Represents the response for Ethereum with an `ethereum` field containing a `Price`.
- **SP500Price**: Represents the response for the S&P 500 index, which contains a `chart` with a `result` that holds the market price.

Each of these structures is used to parse the JSON data into Rust structs that can be used by the program.

### 2. `assets.rs`
This module contains the definitions for the asset structs and logic to fetch and save their prices:
- **Bitcoin, Ethereum, and SP500 structs**: These structs represent the different assets for which the prices will be fetched. Each asset contains an `api_address` for the API endpoint, a `file_name` to save the prices to, and a `cached_price` to store the latest fetched price.
- **Pricing Trait**: The `Pricing` trait defines the common behavior for fetching prices (`fetch_price`) and saving them to a file (`save_to_file`). Each asset implements this trait, providing specific logic for fetching data from the relevant API and saving it to the specified file.
- **Helper functions**: There are also helper functions for fetching the price from an API (`fetch_price`) and saving the data to a file (`save_price_to_file`).

### 3. `main.rs`
This is the entry point of the application. It initializes a list of assets to be fetched with their respective API endpoints and output files. The program runs in an infinite loop, fetching the latest price for each asset, saving the result to the corresponding file every 10 seconds.

## Running
Execute `cargo run` from the terminal in the current directory (same as README).
