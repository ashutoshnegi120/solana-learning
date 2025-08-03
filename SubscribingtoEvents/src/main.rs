use anyhow::Result;
use futures::StreamExt;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, signature::Signer,
    signer::keypair::Keypair,
};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://127.0.0.1:8899".to_string();
    let ws_url = "ws://127.0.0.1:8900".to_string();
    let wallet = Keypair::new();
    let pub_key = wallet.pubkey();

    let connect = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());
    let ws_client = PubsubClient::new(&ws_url).await?;

    tokio::spawn(async move {
        let config = RpcAccountInfoConfig {
            encoding: None,
            data_slice: None,
            min_context_slot: None,
            commitment: Some(CommitmentConfig::confirmed()),
        };

        let (mut stream, _) = ws_client
            .account_subscribe(&pub_key, Some(config))
            .await
            .expect("Failed to subscribe to account updates");

        while let Some(account) = stream.next().await {
            println!("{:#?}", account);
        }
    });
    let airdrop_signature = connect.request_airdrop(&pub_key, LAMPORTS_PER_SOL).await?;
    loop {
        let confirmed = connect.confirm_transaction(&airdrop_signature).await?;
        if confirmed {
            break;
        }
    }
    Ok(())
}
