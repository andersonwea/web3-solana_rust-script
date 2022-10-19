use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    str::FromStr,
};

use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_program::{lamports, system_instruction};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;

// Read wallets from a whitelist file
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    // Whitelist file
    let whitelist = lines_from_file("/home/chain/WEB3_Solana/transfer_sol/wallets.txt")
        .expect("Could not Open file");

    // Private key
    let f_secret_key: [u8; 64] = [
        216, 190, 216, 99, 147, 214, 230, 63, 49, 71, 205, 91, 80, 196, 252, 40, 177, 6, 219, 160,
        6, 167, 88, 89, 237, 84, 253, 80, 181, 87, 104, 82, 111, 32, 13, 41, 235, 211, 76, 22, 74,
        169, 144, 140, 83, 90, 110, 56, 136, 132, 187, 77, 179, 206, 153, 105, 57, 252, 155, 183,
        111, 250, 182, 133,
    ];

    // Amount SOL to send
    let lamports_to_send = 100_000_000; //0.1 SOL 1 SOL = 1_000_000_000

    let rpc_url = String::from("https://api.testnet.solana.com");
    let connection = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let recent_blockhash = connection
        .get_latest_blockhash()
        .expect("Failed to get latest blockhash.");

    for wallet in whitelist {
        if let Ok(from) = Keypair::from_bytes(&f_secret_key) {
            let frompubkey: Pubkey = Signer::pubkey(&from);

            let wallet_b = &wallet;
            let topubkey: Pubkey = Pubkey::from_str(&wallet_b).unwrap();

            let ix = system_instruction::transfer(&frompubkey, &topubkey, lamports_to_send);

            let txn = Transaction::new_signed_with_payer(
                &[ix],
                Some(&frompubkey),
                &[&from],
                recent_blockhash,
            );

            match connection.send_and_confirm_transaction(&txn) {
                Ok(sig) => loop {
                    if let Ok(confirmed) = connection.confirm_transaction(&sig) {
                        if confirmed {
                            println!("Transaction: {} Status: {} \n", sig, confirmed);
                            println!(
                                "Sending {} SOL for [{}] ",
                                lamports_to_send as f64 / 1_000_000_000.0,
                                &wallet
                            );
                            break;
                        }
                    }
                },
                Err(e) => println!("Error transfering Sol:, {}", e),
            }
        }
    }
}
