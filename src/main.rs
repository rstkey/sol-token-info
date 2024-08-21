// Import necessary modules from the Solana client and related libraries
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use std::sync::Arc;
use tokio::task;
use spl_token::state::Mint;
use solana_program::program_pack::Pack;

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

    println!("{}", token_info.supply);

}