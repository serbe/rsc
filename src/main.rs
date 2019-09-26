// use std::io::{Error, ErrorKind};
// use std::net::SocketAddr;

// use futures::sink::Sink;
use futures::future::Future;
use tokio::io::{read_exact, write_all};
use tokio::net::TcpStream;
// use tokio::codec::{FramedWrite, LengthDelimitedCodec};
// use serde_json::{Value};
// use tokio_serde_json::WriteJson;

use client::RpcClient;
use errors::Result;

mod client;
mod errors;
mod messages;

// fn send_message(socket: &TcpStream, msg: Value) -> impl Future<Item = dyn Send<Value>, Error = std::io::Error> {
//     let length_delimited = FramedWrite::new(socket, LengthDelimitedCodec::new());
//     let serialized = WriteJson::new(length_delimited);
//     serialized.send(msg)
// }

fn main() -> Result<()> {
    let addr = "127.0.0.1:10001".parse().unwrap();
    let client = RpcClient::connect(addr);

    client
        .and_then(move |client| client.get(10))
        .and_then(|(value, _)| {
            println!("{:?}", value);
            Ok(())
        })
        .wait()?;

    // let handshake = client
    //     .and_then(|socket| {
    //         println!("successfully connected to {}", socket.local_addr().unwrap());
    //         // The initial greeting from the client
    //         //      field 1: version, 1 byte (0x01 for this version)
    //         //      field 2: number of authentication methods supported, 1 byte
    //         write_all(socket, [1u8, 0u8])
    //     })
    //     .and_then(|(socket, result)| {
    //         println!("wrote to stream; success={:?}", result);
    //         read_exact(socket, [0u8; 2])
    //     })
    //     .and_then(|(socket, buf)| match buf {
    //         [1u8, 0u8] => {
    //             println!("handshake ok");
    //             Ok(socket)
    //         }
    //         _ => Err(Error::from(ErrorKind::InvalidData)),
    //     });

    // tokio::run(client);
    Ok(())
}
