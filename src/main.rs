// Import necessary modules from the Solana client and related libraries
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::str::FromStr;

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

    println!("{}", token_address);

}