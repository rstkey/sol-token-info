// Import necessary modules from the Solana client and related libraries
use dns_lookup::lookup_host;
use mpl_token_metadata::accounts::Metadata;
use solana_client::rpc_client::RpcClient;
use solana_program::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use spl_token::state::Mint;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tokio::task;
use url::Url;

#[tokio::main]
async fn main() {
    // Get the token address from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <token_address>", args[0]);
        return;
    }
    let token_address = &args[1];

    // Convert the token address string to a Pubkey
    let token_pubkey = match Pubkey::from_str(token_address) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            eprintln!("Invalid token address.");
            return;
        }
    };

    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = Arc::new(RpcClient::new(rpc_url));

    // Fetch token information asynchronously
    let client_clone = Arc::clone(&client);
    let token_info = match task::spawn_blocking(move || {
        let mint_account = client_clone.get_account(&token_pubkey).unwrap();
        Mint::unpack(&mint_account.data).unwrap()
    })
    .await
    {
        Ok(info) => info,
        Err(_) => {
            eprintln!("Failed to fetch token information.");
            return;
        }
    };

    // Fetch metadata for the token to get the name, symbol, and website
    let client_clone = Arc::clone(&client);
    let metadata_pubkey = Metadata::find_pda(&token_pubkey).0;
    let metadata_account = match client_clone.get_account(&metadata_pubkey) {
        Ok(account) => account,
        Err(_) => {
            eprintln!("Failed to fetch metadata account.");
            return;
        }
    };

    // Deserialize the metadata account data
    let metadata = match Metadata::safe_deserialize(&metadata_account.data) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Failed to deserialize metadata.");
            return;
        }
    };

    // Extract token information from the metadata
    let token_name = metadata.name;
    let token_symbol = metadata.symbol;
    let token_uri = metadata.uri;

    // Print token information
    println!("Token name: {}", token_name);
    println!("Token symbol: {}", token_symbol);
    println!("Supply: {}", token_info.supply);

    // Fetch additional metadata from the token URI
    let hsmp_uri_metadata = match reqwest::get(&token_uri).await {
        Ok(resp) => match resp.json::<HashMap<String, String>>().await {
            Ok(json) => json,
            Err(_) => {
                eprintln!("Failed to parse JSON metadata.");
                return;
            }
        },
        Err(err) => {
            println!("Failed to request from uri of metadata: {}", err);
            return;
        }
    };

    // Check if the website field exists in the metadata
    match hsmp_uri_metadata
        .get("website")
        .or(hsmp_uri_metadata.get("site"))
    {
        Some(site) => {
            let website_hostname = match Url::parse(&site) {
                Ok(parsed_url) => parsed_url.host_str().unwrap_or("").to_string(),
                Err(_) => {
                    eprintln!("Failed to parse website URL.");
                    return;
                }
            };

            // Print the website information
            println!("Website: {}", site);
            println!("Domain name: {}", website_hostname);

            // Fetch DNS records asynchronously if the website exists
            let dns_records =
                match task::spawn_blocking(move || match lookup_host(&website_hostname) {
                    Ok(ips) => ips,
                    Err(err) => {
                        eprintln!("Failed to lookup DNS records: {}", err);
                        vec![]
                    }
                })
                .await
                {
                    Ok(records) => records,
                    Err(_) => {
                        eprintln!("Failed to fetch DNS records.");
                        return;
                    }
                };

            // Print the number of DNS records found
            println!("Number of Website DNS Records : {}", dns_records.len());
            // Print each IP address found
            for ip in dns_records {
                println!("IP Address: {}", ip);
            }
        }
        _ => {
            eprintln!("Website not found in metadata");
            return;
        }
    };
}
