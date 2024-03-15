//! Example of signing and sending a transaction using a Ledger device.

use alloy_network::EthereumSigner;
use alloy_primitives::{address, U256};
use alloy_provider::{Provider, ProviderBuilder, RootProvider};
use alloy_rpc_client::RpcClient;
use alloy_rpc_types::request::TransactionRequest;
use alloy_signer_ledger::{HDPath, LedgerSigner};
use alloy_transport_http::Http;
use eyre::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    // Instantiate the application by acquiring a lock on the Ledger device.
    let signer = LedgerSigner::new(HDPath::LedgerLive(0), Some(1)).await?;

    // Create a provider with the signer and the network.
    let http = Http::<Client>::new("http://localhost:8545".parse()?);

    // Initialize the provider.
    let provider = ProviderBuilder::new()
        .signer(EthereumSigner::from(signer))
        .provider(RootProvider::new(RpcClient::new(http, true)));

    // Create a transaction.
    let tx = TransactionRequest {
        value: Some(U256::from(100)),
        to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
        gas_price: Some(U256::from(20e9)),
        gas: Some(U256::from(21000)),
        ..Default::default()
    };

    // Broadcast the transaction and wait for the receipt.
    let receipt = provider.send_transaction(tx).await?.with_confirmations(3).get_receipt().await?;

    println!("Send transaction: {:?}", receipt.transaction_hash.unwrap_or_default());

    Ok(())
}
