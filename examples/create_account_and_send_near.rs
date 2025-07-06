use anyhow::Result;
use near_api::{
    self, signer, Account, AccountId, NearToken, NetworkConfig, RPCEndpoint, Signer, Tokens,
};
use near_sandbox_utils::{Sandbox, DEFAULT_GENESIS_ACCOUNT, DEFAULT_GENESIS_ACCOUNT_PRIVATE_KEY};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let sandbox = Sandbox::start_sandbox().await?;
    let network_config = NetworkConfig {
        network_name: "sandbox".to_string(),
        rpc_endpoints: vec![RPCEndpoint::new(sandbox.rpc_addr.parse()?)],
        ..NetworkConfig::testnet()
    };

    let genesis_account_id: AccountId = DEFAULT_GENESIS_ACCOUNT.parse()?;
    let genesis_signer: Arc<Signer> = Signer::new(Signer::from_secret_key(
        DEFAULT_GENESIS_ACCOUNT_PRIVATE_KEY.parse()?,
    ))?;

    let new_account_id: AccountId = format!("{}.{}", "bob", DEFAULT_GENESIS_ACCOUNT).parse()?;
    let new_account_secret_key = signer::generate_secret_key()?;

    Account::create_account(new_account_id.clone())
        .fund_myself(genesis_account_id.clone(), NearToken::from_near(1))
        .public_key(new_account_secret_key.public_key())?
        .with_signer(genesis_signer.clone())
        .send_to(&network_config)
        .await?;

    Tokens::account(genesis_account_id.clone())
        .send_to(new_account_id.clone())
        .near(NearToken::from_near(1))
        .with_signer(genesis_signer)
        .send_to(&network_config)
        .await?;

    let genesis_account_balance = Tokens::account(genesis_account_id.clone())
        .near_balance()
        .fetch_from(&network_config)
        .await?;

    let new_account_balance = Tokens::account(new_account_id.clone())
        .near_balance()
        .fetch_from(&network_config)
        .await?;

    println!("Genesis Balance: {}", genesis_account_balance.total);

    // We expect to see 2 NEAR in Bob's account. 1 NEAR from create_account and 1 NEAR from send_near.
    println!("Bob balance: {}", new_account_balance.total);

    Ok(())
}
