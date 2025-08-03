use std::str::FromStr;

use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://api.devnet.solana.com".to_string(); // instead of localhost
    let connect = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());

    let public_key = Pubkey::from_str("FkGjz8YRVDpd1dDk3WuZjE85eyYEnv2oPMWawV5wFRJp")?;

    let airdrop = connect
        .request_airdrop(&public_key, LAMPORTS_PER_SOL)
        .await?;

    loop {
        let comformed = connect.confirm_transaction(&airdrop).await?;
        if comformed {
            break;
        }
    }
    let balance = connect.get_balance(&public_key).await?;
    println!("Balance: {} SOL", balance as f64 / LAMPORTS_PER_SOL as f64);

    Ok(())
}
