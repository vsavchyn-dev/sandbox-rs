# NEAR Sandbox Examples

This directory contains examples demonstrating how to use the NEAR Sandbox for local development and testing.

## Examples

### Custom Genesis Configuration

[`custom_genesis.rs`](./custom_genesis.rs) - Shows how to customize the sandbox configuration, including:

- Extending the genesis configuration with custom settings
- Adding custom accounts with predefined balances and keys

### Default Sandbox Launch and Creating New Account

[`create_account_and_send_near.rs`](./create_account_and_send_near.rs) - Demonstrates how to launch sandbox with default settings and use `near-api-rs` to:

- Create a new account on the local NEAR Sandbox
- Transfer NEAR tokens between accounts
- Query account balances and basic state via the sandbox RPC

## Running Examples

To run an example:

```bash
cargo run --example custom_genesis
```
