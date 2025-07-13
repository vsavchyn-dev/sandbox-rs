# NEAR Sandbox Examples

This directory contains examples demonstrating how to use the NEAR Sandbox for local development and testing.

## Examples

### Custom Genesis Configuration

[`custom_config.rs`](./custom_config.rs) - Shows how to customize the sandbox configuration, including:

- Extending the genesis configuration with custom settings
- Adding custom accounts with predefined balances and keys
- Setting the RPC port to run on

### Querying default genesis

[`query_default_genesis.rs`] - Demonstrates how to launch the default sandbox configuration and verify genesis state:

- Starting a sandbox with default settings
- Connecting to the sandbox RPC endpoint using near-api-rs
- Querying the default genesis account balance and public key
- Validating that the sandbox launched with expected default values

### Default Sandbox Launch and Creating New Account

[`create_account_and_send_near.rs`](./create_account_and_send_near.rs) - Shows how to launch sandbox with default settings and use it with `near-api-rs` to:

- Create a new account on the local NEAR Sandbox
- Transfer NEAR tokens between accounts
- Query account balances and basic state via the sandbox RPC

## Running Examples

To run an example:

```bash
cargo run --example custom_genesis
```
