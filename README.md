# Solana Token Info CLI

![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![Tokio](https://img.shields.io/badge/Async-Tokio-blue.svg)
![Solana](https://img.shields.io/badge/Blockchain-Solana-green.svg)

## Introduction

**Solana Token Info CLI** is a Rust-based command-line interface (CLI) tool that enables users to input a Solana token address and retrieve detailed information about the token. This tool fetches and displays the following information:

- Token Name
- Token Symbol
- Total Supply
- Website
- Number of Website DNS Records

This project demonstrates the use of Rust’s asynchronous capabilities and error handling features, making it efficient and resilient.

## Features

- **Fetch Token Metadata**: Retrieve the token name, symbol, and total supply.
- **Website Information**: Display the token's associated website and its DNS records.
- **Async Execution**: Uses Tokio for concurrent operations, improving performance.
- **Error Handling**: Graceful handling of errors with meaningful feedback to the user.

## Usage

### Prerequisites

- **Rust**: Ensure that you have Rust and Cargo installed. If not, install it from [here](https://www.rust-lang.org/).

### Steps

1. Clone the repository:

    ```sh
    git clone https://github.com/rust-solman/sol-token-info.git
    cd sol-token-info
    ```

2. Build the project:

    ```sh
    cargo build --release
    ```

3. Run the executable:

    ```sh
    ./target/release/sol-token-info <token_address>
    ```

### Example

  ```sh
  ./target/release/sol-token-info jtojtomepa8beP8AuQc6eXt5FriJwfFMwQx2v2f9mCL
  ```
## Output
The program will output the following information about the token:

- **Token Name**: The name of the token.
- **Token Symbol**: The symbol representing the token.
- **Supply**: The total supply of the token.
- **Website**: The website associated with the token.
- **Number of Website DNS Records**: The number of DNS records found for the token's website.
- **IP Addresses**: The IP addresses associated with the token's website.

#### Sample Output

  ```sh
  Token name: JITO
  Token symbol: JTO
  Supply: 999999924448930418
  Website: https://jito.network
  Domain name: jito.network
  Number of Website DNS Records : 3
  IP Address: 172.67.27.42
  IP Address: 104.22.34.230
  IP Address: 104.22.35.230
  ```

## Contributing
If you find any issues or have suggestions for improvements, feel free to create an issue or submit a pull request.

## Acknowledgements

- The Solana and SPL Token libraries for providing tools to interact with the Solana blockchain.
- The Metaplex Token Metadata library for enabling metadata retrieval.

### Notes
- Ensure the token address is correct to avoid errors.
- DNS records might vary depending on the website and the network setup.
