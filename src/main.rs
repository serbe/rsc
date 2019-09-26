use futures::future::Future;

use client::RpcClient;
use errors::Result;

mod client;
mod errors;
mod messages;

fn main() -> Result<()> {
    let addr = "127.0.0.1:10001".parse().unwrap();
    let client = RpcClient::connect(addr);

    client
        .and_then(|client| client.join())
        .and_then(|client| {
            println!("join");
            Ok(client)
        }).and_then(|client| client.get(10))
        .and_then(|(value, client)| {
            println!("urls {:?}", value);
            Ok(client)
        }).and_then(|client| {
            client.get_anon(20)
        }).and_then(|(value, client)| {
            println!("anon urls {:?}", value);
            Ok(client)
        }).and_then(|client| {
            client.set(Vec::new())
        }).and_then(|_client| {
            println!("set");
            Ok(())
        })
        .wait()?;

    Ok(())
}
