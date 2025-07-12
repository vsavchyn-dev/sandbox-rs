use anyhow::Result;
use near_api::{Account, AccountId, NetworkConfig, RPCEndpoint};
use near_sandbox_utils::high_level::config::{
    DEFAULT_GENESIS_ACCOUNT, DEFAULT_GENESIS_ACCOUNT_BALANCE, DEFAULT_GENESIS_ACCOUNT_PUBLIC_KEY,
};
use near_sandbox_utils::Sandbox;

#[tokio::main]
async fn main() -> Result<()> {
    let sandbox = Sandbox::start_sandbox().await.unwrap();
    let network = NetworkConfig {
        network_name: "sandbox".to_string(),
        rpc_endpoints: vec![RPCEndpoint::new(sandbox.rpc_addr.parse().unwrap())],
        ..NetworkConfig::testnet()
    };

    let genesis_account: AccountId = DEFAULT_GENESIS_ACCOUNT.parse().unwrap();

    let genesis_account_amount = Account(genesis_account.clone())
        .view()
        .fetch_from(&network)
        .await
        .unwrap()
        .data
        .amount;

    let genesis_account_public_key = Account(genesis_account.clone())
        .list_keys()
        .fetch_from(&network)
        .await
        .unwrap()
        .keys
        .first()
        .unwrap()
        .public_key
        .clone();

    assert!(genesis_account_amount == DEFAULT_GENESIS_ACCOUNT_BALANCE);
    assert!(genesis_account_public_key.to_string() == DEFAULT_GENESIS_ACCOUNT_PUBLIC_KEY);

    Ok(())
}
