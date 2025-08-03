use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::{commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL};
use std::{env, fs, path};

fn load_keypair() -> Result<Keypair> {
    let home_path = env::var_os("HOME").unwrap();
    let default_keypair_path = "hiwNjGsbk84ZvxVzDg88W8rRZs62SyFPsmM5aaabmSz.json"; // ! update if you want to use a different path
    let default_keypair_path = path::PathBuf::from(home_path).join(default_keypair_path);

    let keypair_file = fs::read_to_string(default_keypair_path)?;
    let keypair_bytes: Vec<u8> = serde_json::from_str(&keypair_file)?;
    let default_keypair = Keypair::from_bytes(&keypair_bytes)?;
    println!("loaded keypair address -> {:?}", default_keypair.pubkey()); // ! debug

    Ok(default_keypair)
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://127.0.0.1:8899".to_string();
    let client = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());

    let wallet = load_keypair()?;
    let transaction_signature = client
        .request_airdrop(&wallet.pubkey(), 5 * LAMPORTS_PER_SOL)
        .await?;
    loop {
        if client.confirm_transaction(&transaction_signature).await? {
            break;
        }
    }

    let balance = client.get_balance(&wallet.pubkey()).await?;
    println!("Account Balance: {}", balance / LAMPORTS_PER_SOL);
    println!(
        "Transaction Signature: https://explorer.solana.com/tx/{}?cluster=custom",
        transaction_signature
    );

    Ok(())
}
