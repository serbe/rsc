use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Join,
    Get(i64),
    GetAnon(i64),
    Set(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Join,
    Set,
    Err(String),
    Urls(Vec<String>),
}
