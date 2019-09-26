use std::io;
use std::net::SocketAddr;

use futures::sink::Sink;
use tokio::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use tokio::io::{ReadHalf, WriteHalf, AsyncRead};
use tokio::net::TcpStream;
use futures::future::Future;
use tokio_serde_json::{ReadJson, WriteJson};
use futures::stream::Stream;

use crate::messages::{Request, Response};
use crate::errors::RpcError;

pub struct RpcClient {
    read_json: ReadJson<FramedRead<ReadHalf<TcpStream>, LengthDelimitedCodec>, Response>,
    write_json: WriteJson<FramedWrite<WriteHalf<TcpStream>, LengthDelimitedCodec>, Request>,
}

impl RpcClient {
    pub fn connect(addr: SocketAddr) -> impl Future<Item = Self, Error = RpcError> {
        TcpStream::connect(&addr)
            .map(|tcp| {
                let (read_half, write_half) = tcp.split();
                let read_json =
                    ReadJson::new(FramedRead::new(read_half, LengthDelimitedCodec::new()));
                let write_json =
                    WriteJson::new(FramedWrite::new(write_half, LengthDelimitedCodec::new()));
                RpcClient {
                    read_json,
                    write_json,
                }
            })
            .map_err(|e| e.into())
    }

    pub fn get(self, num: i64) -> impl Future<Item = (Vec<String>, Self), Error = RpcError> {
        self.send_request(Request::Get(num))
            .and_then(move |(resp, client)| match resp {
                Some(Response::Urls(value)) => Ok((value, client)),
                Some(Response::Err(msg)) => Err(RpcError::StringError(msg)),
                Some(_) => Err(RpcError::StringError("Invalid response".to_owned())),
                None => Err(RpcError::StringError("No response received".to_owned())),
            })
    }

    pub fn get_anon(self, num: i64) -> impl Future<Item = (Vec<String>, Self), Error = RpcError> {
        self.send_request(Request::Get(num))
            .and_then(move |(resp, client)| match resp {
                Some(Response::Urls(value)) => Ok((value, client)),
                Some(Response::Err(msg)) => Err(RpcError::StringError(msg)),
                Some(_) => Err(RpcError::StringError("Invalid response".to_owned())),
                None => Err(RpcError::StringError("No response received".to_owned())),
            })
    }

    pub fn set(self, key: String, values: Vec<String>) -> impl Future<Item = Self, Error = RpcError> {
        self.send_request(Request::Set(values))
            .and_then(move |(resp, client)| match resp {
                Some(Response::Set) => Ok(client),
                Some(Response::Err(msg)) => Err(RpcError::StringError(msg)),
                Some(_) => Err(RpcError::StringError("Invalid response".to_owned())),
                None => Err(RpcError::StringError("No response received".to_owned())),
            })
    }

    fn send_request(
        self,
        req: Request,
    ) -> impl Future<Item = (Option<Response>, Self), Error = RpcError> {
        let read_json = self.read_json;
        self.write_json
            .send(req)
            .and_then(move |write_json| {
                read_json
                    .into_future()
                    .map(move |(resp, read_json)| {
                        let client = RpcClient {
                            read_json,
                            write_json,
                        };
                        (resp, client)
                    })
                    .map_err(|(err, _)| err)
            })
            .map_err(|e| e.into())
    }
}