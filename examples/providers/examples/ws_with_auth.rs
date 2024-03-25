//! Example of using the WS provider with auth to subscribe to new blocks.

use alloy::{
    network::Ethereum,
    providers::{Provider, RootProvider},
    rpc::client::RpcClient,
    transports::Authorization,
};
use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "wss://your-ws-endpoint.com/";

    let auth = Authorization::basic("username", "password");
    let auth_bearer = Authorization::bearer("bearer-token");

    let ws_transport_basic = WsConnect::with_auth(rpc_url, Some(auth));

    let ws_transport_bearer = WsConnect::with_auth(rpc_url, Some(auth_bearer));

    let rpc_client_basic = RpcClient::connect_pubsub(ws_transport_basic).await?;

    let rpc_client_bearer = RpcClient::connect_pubsub(ws_transport_bearer).await?;

    let provider_basic = RootProvider::<Ethereum, _>::new(rpc_client_basic);

    let provider_bearer = RootProvider::<Ethereum, _>::new(rpc_client_bearer);

    let sub_basic = provider_basic.subscribe_blocks();
    let sub_bearer = provider_bearer.subscribe_blocks();

    let mut stream_basic = sub_basic.await?.into_stream().take(4);
    let mut stream_bearer = sub_bearer.await?.into_stream().take(4);

    println!("Awaiting blocks...");

    // Spawning the block processing for basic auth as a new task
    let basic_handle = tokio::spawn(async move {
        while let Some(block) = stream_basic.next().await {
            println!("From basic {:?}", block.header.number);
        }
    });

    // Similarly for bearer auth
    let bearer_handle = tokio::spawn(async move {
        while let Some(block) = stream_bearer.next().await {
            println!("From bearer {:?}", block.header.number);
        }
    });

    // Wait for both tasks to complete
    let _ = tokio::try_join!(basic_handle, bearer_handle)?;

    Ok(())
}
